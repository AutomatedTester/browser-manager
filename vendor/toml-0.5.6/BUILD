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
    "notice",  # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

# Unsupported target "decode" with type "example" omitted
# Unsupported target "enum_external" with type "example" omitted
# Unsupported target "enum_external_deserialize" with type "test" omitted

rust_library(
    name = "toml",
    srcs = glob(["**/*.rs"]),
    crate_features = [
        "default",
    ],
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.6",
    deps = [
        "//vendor/serde-1.0.111:serde",
    ],
)

# Unsupported target "toml2json" with type "example" omitted
