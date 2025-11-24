use anyhow: :Result;
use cudarc::driver:{CudaDevice, CudaSlice, CudaModule, CudaFunction, LaunchConfig};
use half::f16;
use std::sync::Arc;

#cfg(feature = "gpu")
pub struct Fp16SincosModule {
    device: Arc<CudaDevice>,
    module: CudaModule,
    func: CudaFunction,
}

#cfg(feature = "gpu")
imp Fp16SincosModule {
    pub fn new() -> Result<Self> {
+        let device = Arc::new(CudaDevice::new(0)?);
        let ptx_src = include_str("