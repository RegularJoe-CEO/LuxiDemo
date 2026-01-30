#!/usr/bin/env python3
import requests, hashlib, json

ENDPOINT = "http://localhost:10000/evaluate"
TESTS = [
    ("sin(x)*cos(x)", [0.5,1.0,1.57,2.0,3.14], "e4f0bae37c4150f642e7ecb0983e72e35731fce3085457ec2459eff8a19f338d"),
    ("sin(x)", [0.5,1.0,1.57], "6aa5d30189d51808836ac5760daa9781b79889b46b0086614bafe6a4dab86713"),
    ("cos(x)", [0.5,1.0,1.57], "b7c1f8996c8ac9ccd78152c9059de701e0ad9b92cd3b34a6bc3aad6e29118920"),
    ("exp(x)", [0.5,1.0,2.0], "850398cb9aa7804013779dfbe9f3e3af8626dfccd8861d7b04bc407b5ca85425"),
    ("ln(x)", [0.5,1.0,2.0], "5c9f7a02bf2b9495332e4a8c55d56d4cf402c4d542aaeca621bd4962dbbedd65"),
    ("sqrt(x)", [0.5,1.0,4.0], "2d1b204c60f1e52f4ff35d720a164aa4c7a088aa903f8c4ca1c9ef00609b8033"),
    ("x^2", [0.5,1.0,2.0], "65a7d42468848d2103de850716f0fbcf99ec512d929899fdb42fc15323f1a882"),
    ("x^3", [0.5,1.0,2.0], "4c176c9025305f8a581e212d2e51ba3ee7092ff7cb54f86e7faa99609e56f83c"),
    ("erf(x)", [0.5,1.0,1.5], "a3c9114b35331843254ee92e00697f5a8f79fc8f854b2f2e186c41b0c3dd6729"),
    ("normcdf(x)", [0.5,1.0,1.5], "466726456b025149e1f51aeb037fd1b1d74ef35ad50caa3ce4600afad62f518e"),
    ("normpdf(x)", [0.5,1.0,1.5], "ca921dc59661a1372923f7851d8d3afdeb0725070769a4986aad41174405a57a"),
    ("gamma(x)", [0.5,1.0,2.5], "64af395a9d8c995404f56254fae96d1dc4a5c678262875fa00e89ef1a1fd963e"),
]

passed = 0
for expr, values, expected_hash in TESTS:
    resp = requests.post(ENDPOINT, json={"expr":expr,"values":values,"precision":"f32"})
    data = resp.json()
    if data.get("sha256") == expected_hash:
        print(f"✅ {expr}: PASS")
        passed += 1
    else:
        print(f"❌ {expr}: FAIL - got {data.get('sha256')[:16]}... expected {expected_hash[:16]}...")

print(f"\n🎯 RESULT: {passed}/{len(TESTS)} FUNCTIONS OPERATIONAL")
