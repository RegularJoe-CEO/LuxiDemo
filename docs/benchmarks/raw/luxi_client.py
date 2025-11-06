import os, time
from typing import Dict, List, Optional, Tuple

# orjson for faster dumps if available
try:
    import orjson as _json
    def _dumps(obj): return _json.dumps(obj)
except Exception:
    import json as _json_fallback
    def _dumps(obj):
        s = _json_fallback.dumps(obj)
        return s.encode("utf-8") if isinstance(s, str) else s

import requests
try:
    import requests_unixsocket
    _HAVE_UNIX = True
except Exception:
    _HAVE_UNIX = False

from requests.adapters import HTTPAdapter

class LuxiClient:
    def __init__(self, base: Optional[str] = None, timeout: float = 10.0):
        self.base = (base or os.environ.get("LUXI_URL", "http://127.0.0.1:8080")).rstrip("/")
        self.transport = "uds" if self.base.startswith("http+unix://") else "tcp"
        self.timeout = timeout
        if self.transport == "uds":
            if not _HAVE_UNIX:
                raise RuntimeError("LUXI_URL is http+unix:// but requests-unixsocket is not installed")
            self.sess = requests_unixsocket.Session()
        else:
            self.sess = requests.Session()
        try:
            ad = HTTPAdapter(pool_connections=32, pool_maxsize=32)
            self.sess.mount("http://", ad); self.sess.mount("https://", ad)
        except Exception:
            pass
        self._eval_choice = None

    def _url(self, path: str) -> str:
        return self.base + path

    def health(self) -> Tuple[int, str]:
        for hp in ("/health", "/healthz", "/status"):
            try:
                r = self.sess.get(self._url(hp), timeout=self.timeout)
                return r.status_code, r.text
            except Exception:
                continue
        return 0, ""

    def _parse_eval(self, obj) -> List[float]:
        if isinstance(obj, list):
            return obj
        if isinstance(obj, dict):
            for k in ("y","ys","results","values","value","result","data"):
                if k in obj:
                    v = obj[k]
                    if isinstance(v, list): return v
                    if isinstance(v, (int,float)): return [float(v)]
            for v in obj.values():
                if isinstance(v, dict):
                    for kk in ("y","ys","results","values","value"):
                        vv = v.get(kk)
                        if isinstance(vv, list): return vv
                        if isinstance(vv, (int,float)): return [float(vv)]
        raise ValueError("Unknown /evaluate response shape")

    def evaluate_batch(self, expr: str, xs: List[float], vars: Optional[Dict[str, float]] = None) -> Tuple[List[float], float]:
        vars = vars or {}
        headers = {"Content-Type":"application/json","Accept-Encoding":"gzip","Connection":"keep-alive"}
        variants = [
            lambda: {"expr": expr, "x": xs, "vars": vars},
            lambda: {"expr": expr, "xs": xs, "vars": vars},
            lambda: {"expression": expr, "x": xs, "vars": vars},
        ]
        if self._eval_choice is not None:
            payload = variants[self._eval_choice]()
            t0 = time.perf_counter()
            r = self.sess.post(self._url("/evaluate"), data=_dumps(payload), headers=headers, timeout=self.timeout)
            t1 = time.perf_counter()
            r.raise_for_status()
            return self._parse_eval(r.json()), (t1 - t0)

        last = None
        for i, build in enumerate(variants):
            try:
                payload = build()
                t0 = time.perf_counter()
                r = self.sess.post(self._url("/evaluate"), data=_dumps(payload), headers=headers, timeout=self.timeout)
                t1 = time.perf_counter()
                if not (200 <= r.status_code < 300):
                    last = (r.status_code, r.text[:200]); continue
                out = self._parse_eval(r.json())
                self._eval_choice = i
                return out, (t1 - t0)
            except Exception as e:
                last = str(e); continue
        raise RuntimeError(f"All /evaluate payload variants failed. Last: {last}")
