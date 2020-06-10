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
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "constructors" with type "bench" omitted

rust_library(
    name = "directories",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//cargo/vendor/cfg-if-0.1.10:cfg_if",
        "//cargo/vendor/dirs-sys-0.3.5:dirs_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "2.0.2",
    crate_features = [
    ],
)

