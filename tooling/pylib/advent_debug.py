# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

from sys import stderr

from rich import print as rprint


def eprint(*args, **kwargs):
    """Wrapper around rich.print that defaults to file=sys.stderr
        Has the same semantics as print. """
    if "file" not in kwargs:
        kwargs["file"] = stderr
    rprint(*args, **kwargs)


eprint(
    f"[red]Loaded [green]{__file__}[/green]."
    " Don't forget to remove debug print statements before committing![/red]"
)
