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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "blake2b_simd",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/arrayref-0.3.6:arrayref",
        "//cargo/vendor/arrayvec-0.5.1:arrayvec",
        "//cargo/vendor/constant_time_eq-0.1.5:constant_time_eq",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.10",
    crate_features = [
        "default",
        "std",
    ],
)

