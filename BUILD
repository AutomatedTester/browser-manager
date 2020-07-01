"""
cargo-raze workspace build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

licenses([
  "notice" # See individual crates for specific licenses
])

rust_binary(
    name="browser-manager",
    srcs=glob(["src/*.rs"]),
    deps=[
        ":clap",
        ":which",
        ":toml",
        ":directories"
    ]
)

alias(
    name = "clap",
    actual = "//vendor/clap-2.33.1:clap",
)
alias(
    name = "directories",
    actual = "//vendor/directories-2.0.2:directories",
)
alias(
    name = "toml",
    actual = "//vendor/toml-0.5.6:toml",
)
alias(
    name = "which",
    actual = "//vendor/which-3.1.1:which",
)
