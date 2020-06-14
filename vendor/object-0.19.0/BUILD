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


# Unsupported target "integration" with type "test" omitted
# Unsupported target "nm" with type "example" omitted
# Unsupported target "objcopy" with type "example" omitted
# Unsupported target "objdump" with type "example" omitted

rust_library(
    name = "object",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2018",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.19.0",
    crate_features = [
        "coff",
        "elf",
        "macho",
        "pe",
        "read_core",
    ],
)

# Unsupported target "parse_self" with type "test" omitted
