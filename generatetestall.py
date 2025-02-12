#!/usr/bin/python3
# SPDX-License-Identifier: BSD-3-Clause
# Copyright 2024-2025 UxuginPython
from itertools import *
import re
from os import system
def powerset(iterable):
    "powerset([1,2,3]) → () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    new_s = []
    for i in s:
        if not i.startswith('internal_'):
            new_s.append(i)
    s = new_s
    return chain.from_iterable(combinations(s, r) for r in range(len(s)+1))
file = open('Cargo.toml')
cargo = file.read()
file.close()
version=re.search(r'(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?', cargo).group(0)
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
test = '\n'.join(i.rstrip() for i in f'#!/bin/bash\n#Generated automatically by rrtk {version}\nset -e\n{'\n'.join(f'echo {' '.join(i)}\ncargo test --no-default-features{' --features' if len(i) > 0 else ''} {','.join(i)}' for i in combinations)}'.split('\n')).strip()+'\n'
check = '\n'.join(i.rstrip() for i in f'#!/bin/bash\n#Generated automatically by rrtk {version}\nset -e\n{'\n'.join(f'echo {' '.join(i)}\ncargo check --no-default-features{' --features' if len(i) > 0 else ''} {','.join(i)}' for i in combinations)}'.split('\n')).strip()+'\n'
file = open('testall.sh', 'w')
file.write(test)
file.close()
system('chmod +x testall.sh')
file = open('checkall.sh', 'w')
file.write(check)
file.close()
system('chmod +x checkall.sh')
