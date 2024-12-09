# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

from sys import stderr
from typing import Any

from rich import print as rprint


def eprint(*args: Any, **kwargs: Any):
    """Wrapper around rich.print that defaults to file=sys.stderr
    Has the same semantics as rich.print (i.e. the same as print, but with
    support for rich console markup.

    https://rich.readthedocs.io/en/stable/markup.html#console-markup"""
    if "file" not in kwargs:
        kwargs["file"] = stderr
    rprint(*args, **kwargs)


eprint(
    f"[red]Loaded [green]{__file__}[/green]."
    " Don't forget to remove debug print statements before committing![/red]"
)
