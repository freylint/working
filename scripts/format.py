#!/usr/bin/env python3
"""Language agnostic code formatting script"""

from common import exec_cmds

exec_cmds(["cargo fmt", "black ./**/*.py"])
