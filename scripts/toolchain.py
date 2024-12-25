from common import exec_cmds

exec_cmds(
    [
        "cargo install cross --git https://github.com/cross-rs/cross --force",
        "cargo install sccache --force",
        "rustup component add rustc-codegen-cranelift-preview --toolchain nightly",
    ]
)
