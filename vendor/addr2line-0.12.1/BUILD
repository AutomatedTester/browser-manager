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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "addr2line" with type "example" omitted

rust_library(
    name = "addr2line",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendor/gimli-0.21.0:gimli",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.12.1",
    crate_features = [
    ],
)

# Unsupported target "correctness" with type "test" omitted
# Unsupported target "output_equivalence" with type "test" omitted
# Unsupported target "parse" with type "test" omitted
