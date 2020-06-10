"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "accuracy" with type "test" omitted
# Unsupported target "backtrace" with type "example" omitted

rust_library(
    name = "backtrace",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/addr2line-0.12.1:addr2line",
        "//cargo/vendor/cfg-if-0.1.10:cfg_if",
        "//cargo/vendor/libc-0.2.71:libc",
        "//cargo/vendor/object-0.19.0:object",
        "//cargo/vendor/rustc-demangle-0.1.16:rustc_demangle",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.48",
    crate_features = [
        "addr2line",
        "default",
        "gimli-symbolize",
        "object",
        "std",
    ],
)

# Unsupported target "benchmarks" with type "bench" omitted
# Unsupported target "concurrent-panics" with type "test" omitted
# Unsupported target "long_fn_name" with type "test" omitted
# Unsupported target "raw" with type "example" omitted
# Unsupported target "skip_inner_frames" with type "test" omitted
# Unsupported target "smoke" with type "test" omitted
