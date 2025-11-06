import csv, json, time, platform
from typing import Dict
class BenchLogger:
    def __init__(self, csv_path: str, meta: Dict):
        self.csv_path = csv_path
        self.f = open(csv_path, "w", newline="")
        self.w = csv.writer(self.f)
        self.w.writerow(["ts_unix", "samples", "elapsed_s"])
        meta_out = dict(meta)
        meta_out["platform"] = {
            "python": platform.python_version(),
            "machine": platform.machine(),
            "processor": platform.processor(),
            "system": platform.system(),
            "release": platform.release(),
        }
        with open(csv_path.replace(".csv", ".meta.json"), "w") as mf:
            json.dump(meta_out, mf, indent=2)
    def log(self, sample_count: int, elapsed_s: float):
        self.w.writerow([f"{time.time():.6f}", sample_count, f"{elapsed_s:.9f}"])
        self.f.flush()
    def close(self):
        self.f.close()
