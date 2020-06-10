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


alias(
  name = "rust_argon2",
  actual = ":argon2",
)

rust_library(
    name = "argon2",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/base64-0.11.0:base64",
        "//cargo/vendor/blake2b_simd-0.5.10:blake2b_simd",
        "//cargo/vendor/constant_time_eq-0.1.5:constant_time_eq",
        "//cargo/vendor/crossbeam-utils-0.7.2:crossbeam_utils",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.7.0",
    crate_features = [
    ],
)

# Unsupported target "integration_test" with type "test" omitted
