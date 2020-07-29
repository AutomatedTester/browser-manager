"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

package(default_visibility = [
    # Public for visibility by "@raze__crate__version//" targets.
    #
    # Prefer access through "//", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "restricted",  # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

# Unsupported target "bail_ensure" with type "example" omitted
# Unsupported target "basic_fail" with type "test" omitted
# Unsupported target "error_as_cause" with type "example" omitted
# Unsupported target "fail_compat" with type "test" omitted

rust_library(
    name = "failure",
    srcs = glob(["**/*.rs"]),
    crate_features = [
        "backtrace",
        "std",
    ],
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.1.8",
    deps = [
        "//vendor/backtrace-0.3.48:backtrace",
    ],
)

# Unsupported target "macro_trailing_comma" with type "test" omitted
# Unsupported target "simple" with type "example" omitted
# Unsupported target "string_custom_error_pattern" with type "example" omitted
