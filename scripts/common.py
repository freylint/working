import os


def exec_cmds(cmds=[]):
    """
    Execute a list of shell commands in order.

    Args:
        cmds (list): List of shell commands to execute. Each command should be a string.
    """
    for command in cmds:
        print(f"{command}:\n")
        os.system(command)
