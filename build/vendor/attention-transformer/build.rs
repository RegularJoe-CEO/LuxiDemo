// build.rs
// attention-transformer CUDA support (optional)

fn nvcc_available() -> bool {
    std::process::Command::new("nvcc")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    println!("cargo::rustc-check-cfg=cfg(cuda_compilation_failed)");
    println!("cargo::rustc-check-cfg=cfg(has_cuda_kernels)");

    if std::env::var("CARGO_FEATURE_CUDA").is_err() {
        return;
    }

    println!("cargo:rerun-if-changed=cuda_src/waller_operator.cu");
    println!("cargo:rerun-if-changed=cuda_src/cuda_extras.cu");
    println!("cargo:rerun-if-env-changed=CUDA_ARCH");

    if !nvcc_available() {
        println!("cargo:warning=nvcc not found — CUDA kernels disabled");
        println!("cargo:rustc-cfg=cuda_compilation_failed");
    } else {
        #[cfg(feature = "cuda")]
        {
            let mut build = cc::Build::new();
            build
                .cuda(true)
                .flag("-std=c++17")
                .flag("-O3")
                .file("cuda_src/waller_operator.cu")
                .file("cuda_src/cuda_extras.cu");

            // H100 (sm_90) + portable sm_70 fallback
            if std::env::var("CUDA_ARCH")
                .map(|a| a.contains("90"))
                .unwrap_or(false)
            {
                build.flag("-gencode=arch=compute_90,code=sm_90");
            } else {
                build
                    .flag("-gencode=arch=compute_70,code=sm_70")
                    .flag("-gencode=arch=compute_90,code=sm_90");
            }

            build.compile("waller_cuda");

            // Dynamic cudart: static libcudart from cc may omit cudaHostFree-era symbols on some pods.
            println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
            println!("cargo:rustc-link-lib=dylib=cudart");
            println!("cargo:rustc-cfg=has_cuda_kernels");
        }
    }
}