use cfg_aliases::cfg_aliases;

fn main() {
    cxx_build::bridge("benches/vec16c_benchmark.rs")
        .file("benches/vcl_benchmark.cpp")
        .flag("-march=native")
        .opt_level(3)
        .std("c++17")
        .compile("vcl_benchmark");

    println!("cargo:rerun-if-changed=benches/vec16c_benchmark.rs");

    cfg_aliases! {
        linux: { target_os = "linux" },
        sse: { target_feature = "sse" },
        no_sse: { not(sse) },
    }
}
