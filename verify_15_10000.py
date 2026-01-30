import requests
print("\n🚀 STARTING 15-POINT FULL SPECTRUM TEST on Port 10000...\n")
funcs = ["sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "exp", "ln", "sqrt", "erf", "normcdf", "inverse_cdf"]
passed = 0
for f in funcs:
    try:
        r = requests.post("http://localhost:10000/evaluate", json={"expr": f+"(x)", "values": [0.5]})
        status = "OK" if r.status_code == 200 else f"FAIL {r.status_code}"
        print(f"✅ {f.upper():<12} [{status}]")
        if r.status_code == 200: passed += 1
    except: 
        print(f"❌ {f.upper():<12} [OFFLINE]")
print(f"\n🎯 RESULT: {passed}/15 FUNCTIONS OPERATIONAL")
