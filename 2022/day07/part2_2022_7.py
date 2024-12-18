#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 7 Part 2

import sys
from typing import TypeAlias, Optional, Union, cast

File: TypeAlias = int
FSObj: TypeAlias = Union["Directory", File]
FSPath: TypeAlias = list[str]


class Directory:
    def __init__(self, name: str, _parent: Optional["Directory"] = None):
        self.name = name
        self._contents: dict[str, FSObj] = {}
        self._size: int = 0
        self._parent = _parent

    def __getitem__(self, name: str) -> FSObj:
        return self._contents[name]

    def __contains__(self, name: str) -> bool:
        return name in self._contents

    def _increase_size(self, size: int):
        self._size += size
        if self._parent is not None:
            cast(Directory, self._parent)._increase_size(size)

    def du(self) -> int:
        return self._size

    def fallocate(self, name: str, size: int):
        if name in self and isinstance(self[name], Directory):
            raise TypeError(f"{name} is a directory.")
        self._contents[name] = size
        self._increase_size(size)

    def mkdir(self, name: str):
        if name in self:
            raise ValueError(f"{name} already exists")
        self._contents[name] = Directory(name, _parent=self)

    def tree(self, nest_level: int = 0) -> str:
        s = "  " * nest_level + f"- {self.name} (dir, total_size={self._size})"
        for name, obj in self._contents.items():
            if isinstance(obj, File):
                s += f"\n{'  ' * (nest_level + 1)}- {name} (file, size={obj})"
            else:
                s += "\n" + cast(Directory, obj).tree(nest_level + 1)
        return s


class FileSystem:
    def __init__(self) -> None:
        self._fstree = Directory("/")
        self._cwd: FSPath = []
        self.dirs: list[FSPath] = [[]]

    def cd(self, directory: str):
        match directory:
            case "..":
                new_dir = self._cwd[:-1]
            case "/":
                new_dir = []
            case dir:
                new_dir = self._cwd + [dir]
        if isinstance(self[new_dir], File):
            raise TypeError(f"/{'/'.join(new_dir)} is a file")
        self._cwd = new_dir

    def __getitem__(self, path: FSPath) -> FSObj:
        if path == ["/"] or path == []:
            return self._fstree
        fsobj: FSObj = self._fstree
        for elem in path:
            if isinstance(fsobj, File):
                raise TypeError
            fsobj = fsobj[elem]
        return fsobj

    def fallocate(self, name: str, size: int):
        cast(Directory, self[self._cwd]).fallocate(name, size)

    def mkdir(self, name: str):
        cast(Directory, self[self._cwd]).mkdir(name)
        self.dirs.append(self._cwd + [name])

    def du(self) -> int:
        return self._fstree.du()

    def tree(self) -> str:
        return self._fstree.tree()

    def size_to_delete(self) -> int:
        used_space = self._fstree.du()
        sizes = (cast(Directory, self[dir]).du() for dir in self.dirs)
        return min(sz for sz in sizes if used_space - sz <= 40_000_000)


def main() -> None:
    fs = FileSystem()
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            if line.startswith("$ cd"):
                fs.cd(line.strip()[5:])
            elif line.startswith("$ ls"):
                pass
            elif line.startswith("dir"):
                fs.mkdir(line.strip()[4:])
            else:
                size, name = line.strip().split(" ", 1)
                fs.fallocate(name, int(size))
    print(fs.size_to_delete())


if __name__ == "__main__":
    main()
