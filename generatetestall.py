#!/usr/bin/python3
# SPDX-License-Identifier: LGPL-3.0-only
#Copyright 2024 UxuginPython on GitHub
#
#     This file is part of Rust Robotics ToolKit.
#
#    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.
#
#    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
#
#    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.

#This file will generate a Bash script for testing all combinations of features using Cargo.
#Its output should already be stored in testall.sh, and you can regenerate this file with `python3 generatetestall.py > testall.sh`.
#You can also run `python3 generatetestall.py | bash` to skip the script and directly run the resulting code. testall.sh will remain unchanged.

from itertools import *
import re
def powerset(iterable):
    "powerset([1,2,3]) → () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))
file = open('Cargo.toml')
cargo = file.read()
file.close()
cargo = cargo.split('[features]')
cargo = cargo[1]
cargo = cargo.split('\n')
cargo = [re.sub(re.compile('\\s'), '', i) for i in cargo]
new_cargo = []
for i in cargo:
    try:
        if i[0] == '[':
            break
        if not re.compile('default\\s*=\\s*').match(i):
            new_cargo.append(i)
    except IndexError:
        pass
cargo = new_cargo
cargo = [i.split('=') for i in cargo]
cargo = {i: set(eval(j)) for i, j in cargo}
features = cargo.keys()
combinations = list(powerset(features))
for i in features:
    for j in cargo[i]:
        new_combinations = []
        for k in enumerate(combinations):
            if not(i in k[1] and j in k[1]):
                new_combinations.append(k[1])
        combinations = new_combinations
print(f'#!/bin/bash\n#Generated automatically by rrtk 0.3.0-alpha.3\nset -e\n{'\n'.join(f'echo {' '.join(i)}\ncargo test --no-default-features{' --features' if len(i) > 0 else ''} {','.join(i)}' for i in combinations)}')
