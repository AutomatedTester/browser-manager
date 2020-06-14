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
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "clap",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendor/ansi_term-0.11.0:ansi_term",
        "//vendor/atty-0.2.14:atty",
        "//vendor/bitflags-1.2.1:bitflags",
        "//vendor/strsim-0.8.0:strsim",
        "//vendor/textwrap-0.11.0:textwrap",
        "//vendor/unicode-width-0.1.7:unicode_width",
        "//vendor/vec_map-0.8.2:vec_map",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "2.33.1",
    crate_features = [
        "ansi_term",
        "atty",
        "color",
        "default",
        "strsim",
        "suggestions",
        "vec_map",
    ],
)

