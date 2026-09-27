#!/usr/bin/env python3
"""Build the fixed prompt set for the drift demo (targets + filler pool), pre-tokenized with the
Qwen2-7B-Instruct tokenizer and its chat template (system prompt "You are a helpful assistant.").
Every engine receives these exact token ids, so tokenization can never be a source of difference.

usage: build_data.py TOKENIZER_DIR   (dir with tokenizer.json; the HF files of Qwen/Qwen2-7B-Instruct)
writes: prompts.json, filler_pool.json next to this script.
Run on the pod, verify_chat_template() cross-checks the hand-rendered template against
transformers.apply_chat_template (pass --verify)."""
import json, os, sys, random, hashlib, re

HERE = os.path.dirname(os.path.abspath(__file__))
SYS = "You are a helpful assistant."

def render(user):  # Qwen2 chat template, add_generation_prompt=True
    return f"<|im_start|>system\n{SYS}<|im_end|>\n<|im_start|>user\n{user}<|im_end|>\n<|im_start|>assistant\n"

def declaration():
    raw = open(os.path.join(HERE, "declaration_of_independence_gutenberg1.txt"), encoding="utf-8").read()
    paras = [re.sub(r"\s+", " ", p).strip() for p in raw.split("\n\n")]
    return "\n\n".join(p for p in paras if p)

TARGETS = [
    # (id, kind, user message)
    ("p01", "factual", "What is the capital of Australia, and why was it chosen instead of Sydney or Melbourne?"),
    ("p02", "factual", "Explain in about 150 words how vaccines train the immune system."),
    ("p03", "factual", "List the planets of the solar system in order from the Sun, with one notable fact about each."),
    ("p04", "factual", "Who wrote 'Pride and Prejudice', and what are the novel's main themes?"),
    ("p05", "factual", "What causes the seasons on Earth? Many people think it is the distance from the Sun; explain why that is wrong."),
    ("p06", "reasoning", "A bat and a ball cost $1.10 in total. The bat costs $1.00 more than the ball. How much does the ball cost? Think step by step."),
    ("p07", "reasoning", "A train leaves at 2:45 PM and the trip takes 3 hours and 50 minutes. What time does it arrive? Show your work."),
    ("p08", "reasoning", "Alice is older than Bob. Bob is older than Carol. Dave is younger than Carol but older than Erin. Who is the second youngest? Explain your reasoning."),
    ("p09", "reasoning", "Pencils cost $0.30 each or $0.75 for a pack of 3. What is the cheapest way to buy exactly 14 pencils, and how much does it cost? Reason step by step."),
    ("p10", "reasoning", "Is 391 a prime number? Work it out step by step."),
    ("p11", "code", "Write a Python function that returns the n-th Fibonacci number iteratively. Include a docstring and two example calls."),
    ("p12", "code", "Write a SQL query that returns the top 3 customers by total order amount, given tables customers(id, name) and orders(id, customer_id, amount). Explain it briefly."),
    ("p13", "code", "What does this JavaScript print, and why?\n\nfor (var i = 0; i < 3; i++) {\n  setTimeout(() => console.log(i), 0);\n}\n\nHow would you change it to print 0, 1, 2?"),
    ("p14", "code", "Write a Bash one-liner that counts the total number of lines in all .py files under the current directory (recursively), then explain each part."),
    ("p15", "code", "Find and fix the bug in this binary search:\n\ndef search(xs, target):\n    lo, hi = 0, len(xs)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if xs[mid] < target:\n            lo = mid\n        else:\n            hi = mid\n    return lo\n"),
    ("p16", "writing", "Write a short, polite email to my landlord asking them to fix a leaking kitchen faucet."),
    ("p17", "writing", "Summarize the pros and cons of remote work for a 20-person software company, as bullet points."),
    ("p18", "writing", "Write an eight-line poem about the ocean at night."),
    ("p19", "writing", "Give three practical tips for improving sleep quality, each with a one-sentence explanation."),
    ("p20", "long_context", None),  # filled below: full Declaration of Independence + question (~1.9k tokens)
]
LONG_Q = ("\n\nBased only on the document above, list five specific grievances against the King, quoting a short phrase "
          "from the text for each, and then state in one sentence what the signers finally declared.")

FILLERS = [
    "Translate 'Where is the train station?' into French, Spanish and German.",
    "Give me a recipe for a simple tomato soup.",
    "What is the difference between a virus and a bacterium?",
    "Write a haiku about autumn leaves.",
    "Explain the rules of chess castling.",
    "How do I reverse a linked list in C?",
    "What were the main causes of World War I?",
    "Suggest five names for a coffee shop by the sea.",
    "Explain what a hash map is to a beginner.",
    "Describe the water cycle for a ten-year-old.",
    "What is the Pythagorean theorem? Give an example.",
    "Write a limerick about a cat who loves boxes.",
    "How does compound interest work?",
    "List the steps to change a flat bicycle tire.",
    "What is the difference between TCP and UDP?",
    "Summarize the plot of Romeo and Juliet in three sentences.",
    "Write a regular expression that matches US ZIP codes.",
    "Why is the sky blue?",
    "Give me a 3-day itinerary for Rome.",
    "Explain gradient descent in plain language.",
    "What are the health benefits of walking daily?",
    "Write a product description for a stainless steel water bottle.",
    "How do tides work?",
    "Convert 72 degrees Fahrenheit to Celsius and show the formula.",
    "Explain the difference between 'affect' and 'effect'.",
    "Write a Python script that reads a CSV file and prints the average of a column.",
    "What is photosynthesis?",
    "Draft a tweet announcing a new bakery opening.",
    "What is the capital of Canada and what is it known for?",
    "Explain Big-O notation with two examples.",
    "Give tips for a job interview at a startup.",
    "What is the difference between weather and climate?",
    "Write a short story opening line set on Mars.",
    "How do noise-cancelling headphones work?",
    "Explain how a bill becomes a law in the United States.",
    "List ten common Spanish verbs with their meanings.",
    "What is inflation and why does it happen?",
    "Write a thank-you note to a teacher.",
    "Explain recursion using a real-world analogy.",
    "What are black holes?",
    "Give me a weekly workout plan for beginners.",
    "Explain the difference between RAM and storage.",
    "Write a JavaScript function that debounces another function.",
    "What causes earthquakes?",
    "Describe the taste of a mango to someone who has never had one.",
    "How do I make a good first impression at a networking event?",
    "What is the theory of evolution in simple terms?",
    "Write a SQL query to count orders per month.",
    "Explain how credit scores work.",
    "What is the function of the mitochondria?",
    "Write a motivational message for someone starting a new job.",
    "How does a refrigerator keep food cold?",
    "Explain the Monty Hall problem.",
    "What are the pros and cons of electric cars?",
    "Write a Go function that checks whether a string is a palindrome.",
    "How are rainbows formed?",
    "Summarize the main ideas of stoicism.",
    "What is machine learning, in two paragraphs?",
    "Give me five fun facts about octopuses.",
    "Explain the difference between git merge and git rebase.",
    "How do vaccines get approved?",
    "Write an apology message for missing a meeting.",
    "What is the greenhouse effect?",
    "Plan a birthday party for a seven-year-old on a small budget.",
]

def main():
    tokdir = sys.argv[1]
    from tokenizers import Tokenizer
    tk = Tokenizer.from_file(os.path.join(tokdir, "tokenizer.json"))
    enc = lambda s: tk.encode(s, add_special_tokens=False).ids
    targets = []
    for pid, kind, msg in TARGETS:
        if msg is None: msg = declaration() + LONG_Q
        text = render(msg); ids = enc(text)
        targets.append({"id": pid, "kind": kind, "user": msg, "chat_text": text, "ids": ids, "n_tokens": len(ids)})
    fill = [{"i": i, "user": m, "ids": enc(render(m))} for i, m in enumerate(FILLERS)]
    # 128 long filler streams (each >= 2400 tokens) = seeded concatenations of chat-formatted filler prompts.
    # Batch-condition filler row = stream[(seed + k) % 128][:S]  (identical rule in the Luxi driver and the vLLM/SGLang drivers)
    rng = random.Random(20260926); streams = []
    for s in range(128):
        ids = []
        while len(ids) < 2400: ids += fill[rng.randrange(len(fill))]["ids"]
        streams.append(ids[:2400])
    P = {"model": "Qwen/Qwen2-7B-Instruct", "system_prompt": SYS, "template": "qwen2 chatml, add_generation_prompt=True", "targets": targets}
    F = {"fillers": fill, "streams": streams, "stream_rule": "row k (k=1..B-1 over non-target rows) uses streams[(seed+k) % 128][:S]"}
    for name, obj in (("prompts.json", P), ("filler_pool.json", F)):
        s = json.dumps(obj, ensure_ascii=False, indent=1); open(os.path.join(HERE, name), "w", encoding="utf-8").write(s)
        print(name, "sha256", hashlib.sha256(s.encode()).hexdigest()[:16])
    for t in targets: print(t["id"], t["kind"], t["n_tokens"])
    print("fillers", len(fill), "lens", min(len(f["ids"]) for f in fill), max(len(f["ids"]) for f in fill))

def verify(tokdir):
    """On the pod: check the hand-rendered template == transformers apply_chat_template, token for token."""
    from transformers import AutoTokenizer
    t = AutoTokenizer.from_pretrained(tokdir)
    P = json.load(open(os.path.join(HERE, "prompts.json"))); bad = 0
    for x in P["targets"]:
        ids = t.apply_chat_template([{"role": "user", "content": x["user"]}], add_generation_prompt=True, tokenize=True)
        if hasattr(ids, "input_ids"): ids = ids["input_ids"]
        if isinstance(ids, dict): ids = ids["input_ids"]
        if list(ids) != x["ids"]: bad += 1; print("MISMATCH", x["id"])
    print("CHAT_TEMPLATE_CHECK", "OK" if bad == 0 else f"{bad} mismatches")
    return bad

if __name__ == "__main__":
    if "--verify" in sys.argv: sys.exit(1 if verify(sys.argv[1]) else 0)
    main()
