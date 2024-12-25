from common import exec_cmds

exec_cmds(
    [
        "cargo b -q",
        "cargo b -rq",
        "cargo test -q",
        "cargo doc",
    ]
)
