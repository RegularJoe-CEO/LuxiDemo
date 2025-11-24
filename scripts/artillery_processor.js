function processRequest(context, ee, next) {
  // Track matrix operations per request
  const opsPerRequest = 72700;  // Your matrix iterations
  context.vars.totalOps = (context.vars.totalOps || 0) + opsPerRequest;
  
  // Custom metric: ops per second
  ee.emit('custom-metric', {
    name: 'matrix_ops_per_sec',
    value: opsPerRequest / context.vars.responseTime / 1000
  });
  
  return next();
}

module.exports = { processRequest };

