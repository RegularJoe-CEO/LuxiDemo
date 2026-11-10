#!/usr/bin/env python3
"""Luxi replay verifier (public, standalone). Needs Python 3.9+ and PyNaCl. No GPU, no model, no engine code.

  verify   BUNDLE --trusted-key KEY             check one signed record against a public key you trust
  compare  A B [--out FILE]                     compare two records token by token and step by step
  compare-dirs DIR_A DIR_B [--out FILE]         same, for two folders of records with the same job ids
  tamper-test BUNDLE --trusted-key KEY [--out]  edit copies of a record and show that verify rejects each edit

KEY is 64 hex characters (an Ed25519 public key) or a file containing them.

What verify checks, for every call in the record: the Ed25519 signature; that the signer is the trusted key; the chain
link to the previous receipt; job id and call index; that the recorded prompt equals the job's prompt; the prompt and
tool-response hashes; the sampling settings (temperature, top_k, top_p_pct, seed, max_new, eos, n_samples) against the
job; the request id derived from the job id; the output-token hash; and the Merkle root and step count recomputed from
every (row, step, token id, 64-bit logit fingerprint). Every output must stop at an end token or at max_new.

What compare reports: whether both records verify on their own, whether every generated token id and every per-step
logit fingerprint is identical, the first call/row/step where they differ, and which signed fields differ (engine and
build identity fields are listed separately from output fields). Exit code 0 only when the outputs are identical.

Limits: a signature shows that the holder of a key signed this record. It does not show which machine ran the job or
that the answer is correct. Agreement with an independent re-run is what shows the record can be reproduced.
Exit codes: 0 = pass, 1 = outputs differ or a record is rejected, 2 = usage or input error.
"""
import argparse, base64, copy, glob, hashlib, json, os, sys

try:
    from nacl.signing import SigningKey, VerifyKey
    from nacl.exceptions import BadSignatureError
except ImportError:
    sys.stderr.write('luxi_verify needs PyNaCl: python3 -m pip install pynacl\n')
    sys.exit(2)

SCHEME = 'luxi-inference-receipt-v1'
PREFIX = 'lxir1_'
MAGIC = b'LXI'
ENGINE_FIELDS = {'engine_sha256', 'engine_mode', 'build_sha256', 'build'}
OUT_FIELDS = ('output_tokens_sha256', 'steps_merkle_root', 'steps_count', 'input_tokens_sha256')
IGNORE = {'run_device', 'parent_receipt_sha256'}


# ---------------- receipt format ----------------
def h(b):
    return hashlib.sha256(b).hexdigest()


def cj(x):
    return json.dumps(x, sort_keys=True, separators=(',', ':')).encode()


def canonical_payload(fields):
    lines = [SCHEME]
    for k in sorted(fields):
        v = str(fields[k])
        if '=' in k or '\n' in k or not k or '\n' in v:
            raise ValueError(f'illegal field {k}')
        lines.append(f'{k}={v}')
    return '\n'.join(lines) + '\n'


def parse_payload(payload):
    ls = payload.split('\n')
    if ls[0] != SCHEME:
        raise ValueError('unknown receipt scheme')
    return dict(l.split('=', 1) for l in ls[1:] if l)


def sign(sk, fields):
    payload = canonical_payload(fields)
    sig = sk.sign(payload.encode()).signature
    pk = bytes(sk.verify_key)
    blob = MAGIC + bytes([1, 0]) + pk + sig
    return dict(receipt=PREFIX + base64.urlsafe_b64encode(blob).decode().rstrip('='), payload=payload,
                payload_sha256=h(payload.encode()), signer_pubkey=pk.hex())


def verify_signature(receipt, payload):
    if not receipt.startswith(PREFIX):
        raise ValueError('bad receipt prefix')
    rest = receipt[len(PREFIX):]
    blob = base64.urlsafe_b64decode(rest + '=' * (-len(rest) % 4))
    if len(blob) != 101 or blob[:3] != MAGIC:
        raise ValueError('bad receipt blob')
    pk, sig = blob[5:37], blob[37:101]
    try:
        VerifyKey(pk).verify(payload.encode(), sig)
    except BadSignatureError:
        raise ValueError('Ed25519 signature rejected')
    return pk.hex()


def leaf_bytes(row, step, token, fp):
    return b'LXSTEP1' + row.to_bytes(4, 'little') + step.to_bytes(4, 'little') + token.to_bytes(4, 'little') + \
        (fp & 0xffffffffffffffff).to_bytes(8, 'little')


def _lh(d):
    return hashlib.sha256(b'\x00' + d).digest()


def _nh(a, b):
    return hashlib.sha256(b'\x01' + a + b).digest()


def _root(hs):
    n = len(hs)
    if n == 1:
        return hs[0]
    k = 1
    while k * 2 < n:
        k *= 2
    return _nh(_root(hs[:k]), _root(hs[k:]))


def mroot(leaves):
    if not leaves:
        return hashlib.sha256(b'').digest()
    return _root([_lh(x) for x in leaves])


def req_id(job_id, ci):
    return int(hashlib.sha256(f'{job_id}:{ci}'.encode()).hexdigest()[:16], 16)


def load_key(arg):
    s = open(arg).read().strip() if os.path.exists(arg) else arg.strip()
    s = s.split()[0].lower()
    if len(s) != 64 or any(c not in '0123456789abcdef' for c in s):
        raise SystemExit(f'error: trusted key must be 64 hex characters (got {s[:20]}...)')
    return s


# ---------------- checks ----------------
def verify(B, trusted=None):
    errs = []
    try:
        job, sp = B['job'], B['job']['sampling']
        eos = set(sp['eos'])
    except (KeyError, TypeError):
        return ['not a Luxi replay record (no job / sampling section)'], []
    if len(B.get('receipts', [])) != len(job['calls']) or len(B.get('calls', [])) != len(job['calls']):
        errs.append(f'count: {len(B.get("receipts", []))} receipts, {len(B.get("calls", []))} calls, job has {len(job["calls"])}')
    parent, prev_out, fields_all = '', None, []
    for ci, (r, c) in enumerate(zip(B.get('receipts', []), B.get('calls', []))):
        tag = f'call {ci}'
        try:
            pk = verify_signature(r['receipt'], r['payload'])
        except (ValueError, KeyError) as e:
            errs.append(f'{tag}: signature: {e}'); pk = None
        if trusted and pk and pk != trusted:
            errs.append(f'{tag}: signed by key {pk[:16]}..., not the trusted key {trusted[:16]}...')
        try:
            f = parse_payload(r['payload'])
        except (ValueError, KeyError) as e:
            errs.append(f'{tag}: payload: {e}'); fields_all.append({}); continue
        fields_all.append(f)
        if f.get('parent_receipt_sha256') != parent:
            errs.append(f'{tag}: chain broken (parent link does not match the previous receipt)')
        parent = h(r['payload'].encode())
        if f.get('call_index') != str(ci) or f.get('job_id') != job['job_id']:
            errs.append(f'{tag}: call index / job id mismatch')
        jc = job['calls'][ci] if ci < len(job['calls']) else {'segments': [], 'n_samples': 0}
        prompt, tool = [], []
        for seg in jc['segments']:
            if seg['kind'] == 'prev_output':
                prompt += prev_out or []
            else:
                prompt += seg['tokens']
                if seg['kind'] == 'tool':
                    tool += seg['tokens']
        if prompt != c['prompt']:
            errs.append(f'{tag}: recorded prompt differs from the job')
        if h(cj(c['prompt'])) != f.get('input_tokens_sha256'):
            errs.append(f'{tag}: input hash mismatch')
        if (h(cj(tool)) if tool else '') != f.get('tool_responses_sha256'):
            errs.append(f'{tag}: tool-response hash mismatch')
        for k in ('temperature', 'top_k', 'top_p_pct', 'seed', 'max_new'):
            if f.get(k) != str(sp[k]):
                errs.append(f'{tag}: {k} signed as {f.get(k)}, job says {sp[k]}')
        if f.get('eos') != ','.join(map(str, sp['eos'])) or f.get('n_samples') != str(jc['n_samples']):
            errs.append(f'{tag}: eos / n_samples mismatch')
        if f.get('request_id') != f'{req_id(job["job_id"], ci):016x}':
            errs.append(f'{tag}: request id mismatch')
        outs, fps = c['outputs'], c['fps']
        if h(cj(outs)) != f.get('output_tokens_sha256'):
            errs.append(f'{tag}: output token hash mismatch')
        try:
            leaves = [leaf_bytes(s, t, outs[s][t], fps[s][t]) for s in range(len(outs)) for t in range(len(outs[s]))]
            if mroot(leaves).hex() != f.get('steps_merkle_root') or str(len(leaves)) != f.get('steps_count'):
                errs.append(f'{tag}: step Merkle root / count mismatch (a token or a logit fingerprint was changed)')
        except (IndexError, OverflowError, AttributeError):
            errs.append(f'{tag}: tokens and fingerprints do not line up')
        for s, o in enumerate(outs):
            if not o or (o[-1] not in eos and len(o) != int(sp['max_new'])) or any(t in eos for t in o[:-1]):
                errs.append(f'{tag} row {s}: output does not stop at an end token or max_new')
        prev_out = outs[0] if outs else []
    return errs, fields_all


def first_divergence(A, B):
    for ci, (ca, cb) in enumerate(zip(A['calls'], B['calls'])):
        if ca['prompt'] != cb['prompt']:
            k = next((i for i, (x, y) in enumerate(zip(ca['prompt'], cb['prompt'])) if x != y), min(len(ca['prompt']), len(cb['prompt'])))
            return dict(call=ci, where='prompt', index=k)
        if len(ca['outputs']) != len(cb['outputs']):
            return dict(call=ci, where='number of samples', a=len(ca['outputs']), b=len(cb['outputs']))
        for s, (oa, ob) in enumerate(zip(ca['outputs'], cb['outputs'])):
            for t in range(max(len(oa), len(ob))):
                ta, tb = (oa[t] if t < len(oa) else None), (ob[t] if t < len(ob) else None)
                fa, fb = (ca['fps'][s][t] if t < len(oa) else None), (cb['fps'][s][t] if t < len(ob) else None)
                if ta != tb or fa != fb:
                    return dict(call=ci, row=s, step=t, token_a=ta, token_b=tb, fingerprint_a=fa, fingerprint_b=fb,
                                tokens_differ=ta != tb, fingerprints_differ=fa != fb)
    if len(A['calls']) != len(B['calls']):
        return dict(where='number of calls', a=len(A['calls']), b=len(B['calls']))
    return None


def compare(A, B):
    ea, fa = verify(A)
    eb, fb = verify(B)
    outd, engd, other = [], [], []
    for ci, (x, y) in enumerate(zip(fa, fb)):
        for k in sorted(set(x) | set(y)):
            if k in IGNORE or x.get(k) == y.get(k):
                continue
            item = f'call {ci}: {k}: {str(x.get(k, "(absent)"))[:24]} vs {str(y.get(k, "(absent)"))[:24]}'
            (engd if k in ENGINE_FIELDS else outd if k in OUT_FIELDS else other).append(item)
    fd = first_divergence(A, B)
    same = not ea and not eb and fd is None and not outd and len(fa) == len(fb) > 0
    rows = toks = 0
    for ca, cb in zip(A['calls'], B['calls']):
        for s, (oa, ob) in enumerate(zip(ca['outputs'], cb['outputs'])):
            rows += 1; toks += (oa == ob and ca['fps'][s] == cb['fps'][s])
    return dict(outputs_identical=same, a_errors=ea, b_errors=eb, first_mismatch=fd,
                answers=rows, answers_identical=toks,
                tokens_a=sum(len(o) for c in A['calls'] for o in c['outputs']),
                tokens_b=sum(len(o) for c in B['calls'] for o in c['outputs']),
                output_field_diffs=outd, engine_or_build_field_diffs=engd, other_signed_field_diffs=other,
                signers=[A['receipts'][0].get('signer_pubkey') if A.get('receipts') else None,
                         B['receipts'][0].get('signer_pubkey') if B.get('receipts') else None],
                devices=[A.get('device'), B.get('device')])


def compare_dirs(da, db):
    rows = []
    fb_all = sorted(glob.glob(os.path.join(db, 'bundle_*.json')))
    fa_names = {os.path.basename(f) for f in glob.glob(os.path.join(da, 'bundle_*.json'))}
    missing = sorted(fa_names - {os.path.basename(f) for f in fb_all})
    S = dict(a=da, b=db, jobs=0, jobs_outputs_identical=0, answers=0, answers_identical=0, tokens_a=0, tokens_b=0,
             missing_in_b=missing, engine_or_build_field_diffs=set(), mismatches=[])
    for fb in fb_all:
        fa = os.path.join(da, os.path.basename(fb))
        if not os.path.exists(fa):
            S['mismatches'].append(dict(file=os.path.basename(fb), problem='no record with this name in A')); continue
        r = compare(json.load(open(fa)), json.load(open(fb)))
        S['jobs'] += 1; S['jobs_outputs_identical'] += r['outputs_identical']
        S['answers'] += r['answers']; S['answers_identical'] += r['answers_identical']
        S['tokens_a'] += r['tokens_a']; S['tokens_b'] += r['tokens_b']
        S['engine_or_build_field_diffs'].update(x.split(': ', 1)[1].split(':')[0] for x in r['engine_or_build_field_diffs'])
        if not r['outputs_identical']:
            S['mismatches'].append(dict(file=os.path.basename(fb), first_mismatch=r['first_mismatch'],
                                        a_errors=r['a_errors'][:3], b_errors=r['b_errors'][:3], output_field_diffs=r['output_field_diffs'][:3]))
    S['engine_or_build_field_diffs'] = sorted(S['engine_or_build_field_diffs'])
    S['all_outputs_identical'] = S['jobs'] > 0 and S['jobs'] == S['jobs_outputs_identical'] and not missing
    return S


def resign_all(B, sk):
    parent = ''
    for ci, (r, c) in enumerate(zip(B['receipts'], B['calls'])):
        f = parse_payload(r['payload'])
        outs, fps = c['outputs'], c['fps']
        leaves = [leaf_bytes(s, t, outs[s][t], fps[s][t]) for s in range(len(outs)) for t in range(len(outs[s]))]
        f.update(output_tokens_sha256=h(cj(outs)), steps_merkle_root=mroot(leaves).hex(), steps_count=len(leaves),
                 input_tokens_sha256=h(cj(c['prompt'])), parent_receipt_sha256=parent)
        B['receipts'][ci] = sign(sk, f)
        parent = B['receipts'][ci]['payload_sha256']


def tamper_test(A, trusted):
    out = []

    def case(name, mut, key=trusted):
        X = copy.deepcopy(A); mut(X)
        e, _ = verify(X, key)
        out.append(dict(test=name, rejected=bool(e), reasons=sorted(set(x.split(': ', 1)[-1] for x in e))[:3]))

    e0, _ = verify(A, trusted)
    out.append(dict(test='untouched record (must be accepted)', rejected=bool(e0), reasons=e0[:3]))
    last = A['calls'][-1]['outputs'][0]
    mid = len(last) // 2

    def tok(X): X['calls'][-1]['outputs'][0][mid] += 1
    def fp(X): X['calls'][0]['fps'][0][len(X['calls'][0]['fps'][0]) // 2] ^= 1
    def prompt(X): X['calls'][0]['prompt'][min(3, len(X['calls'][0]['prompt']) - 1)] += 1
    def seed_payload(X):
        r = X['receipts'][0]; r['payload'] = r['payload'].replace(f"seed={A['job']['sampling']['seed']}\n", 'seed=1\n')
    def seed_job(X): X['job']['sampling']['seed'] = 1
    def temp_job(X): X['job']['sampling']['temperature'] = '0.1'
    def drop_last(X): X['calls'][-1]['outputs'][0] = X['calls'][-1]['outputs'][0][:-1]; X['calls'][-1]['fps'][0] = X['calls'][-1]['fps'][0][:-1]
    def forged(X): X['calls'][-1]['outputs'][0][mid] += 1; resign_all(X, SigningKey.generate())

    case('change one generated token id', tok)
    case('change one per-step logit fingerprint', fp)
    case('change one prompt token', prompt)
    case('change the seed inside the signed receipt', seed_payload)
    case('change the seed in the job only', seed_job)
    case('change the temperature in the job only', temp_job)
    case('drop the last generated token', drop_last)
    case('forged: change a token, recompute every hash, re-sign with a new key (trusted key pinned)', forged)
    wrong = SigningKey.generate().verify_key.encode().hex()
    case('untouched record checked against a wrong trusted key', lambda X: None, key=wrong)
    ok = (not out[0]['rejected']) and all(r['rejected'] for r in out[1:])
    return dict(all_edits_rejected_and_original_accepted=ok, results=out)


def main(argv=None):
    ap = argparse.ArgumentParser(description='Luxi replay verifier (public). Exit 0 = pass, 1 = fail, 2 = usage error.')
    sp = ap.add_subparsers(dest='cmd', required=True)
    v = sp.add_parser('verify'); v.add_argument('bundle'); v.add_argument('--trusted-key', required=True); v.add_argument('--out')
    c = sp.add_parser('compare'); c.add_argument('a'); c.add_argument('b'); c.add_argument('--out')
    d = sp.add_parser('compare-dirs'); d.add_argument('a'); d.add_argument('b'); d.add_argument('--out')
    t = sp.add_parser('tamper-test'); t.add_argument('bundle'); t.add_argument('--trusted-key', required=True); t.add_argument('--out')
    a = ap.parse_args(argv)
    try:
        if a.cmd == 'verify':
            key = load_key(a.trusted_key)
            e, f = verify(json.load(open(a.bundle)), key)
            R = dict(file=a.bundle, trusted_key=key, verified=not e, errors=e, calls=len(f))
            code = 0 if not e else 1
        elif a.cmd == 'compare':
            R = compare(json.load(open(a.a)), json.load(open(a.b))); R['files'] = [a.a, a.b]
            code = 0 if R['outputs_identical'] else 1
        elif a.cmd == 'compare-dirs':
            R = compare_dirs(a.a, a.b); code = 0 if R['all_outputs_identical'] else 1
        else:
            key = load_key(a.trusted_key)
            R = tamper_test(json.load(open(a.bundle)), key); R['file'] = a.bundle; R['trusted_key'] = key
            code = 0 if R['all_edits_rejected_and_original_accepted'] else 1
    except (OSError, json.JSONDecodeError) as e:
        sys.stderr.write(f'error: {e}\n'); return 2
    print(json.dumps(R, indent=1))
    if a.out:
        json.dump(R, open(a.out, 'w'), indent=1)
    return code


if __name__ == '__main__':
    sys.exit(main())
