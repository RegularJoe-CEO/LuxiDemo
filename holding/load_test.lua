request = function()
  local body = [[{"type":"evaluate","expr":"x^2 - 4","x":[3.0,4.0],"vars":{}}]]
  return wrk.format(nil, "/evaluate", {"Content-Type": "application/json"}, body)
end
