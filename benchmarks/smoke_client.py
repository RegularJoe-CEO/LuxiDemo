from luxi_client import LuxiClient
c = LuxiClient()
print("LUXI_URL:", c.base)
print("health:", c.health())
ys, dt = c.evaluate_batch("x*x + 1.0", [0.0, 1.0, 2.0], {})
print("y:", ys, "dt_s:", dt)
