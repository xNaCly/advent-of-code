from part1 import *

def p2_work_temp_group(temp_group: list[str]):
    for s in temp_group[0]:
        if s in temp_group[1] and s in temp_group[2]:
            return priority_lookup.index(s)+1

def p2_work_lines(lines: list[str]):
    temp_group = []
    groups = []
    for i in range(0, len(lines)):
        temp_group.append(lines[i])
        if (i+1) % 3 == 0 and i != 0:
            groups.append(p2_work_temp_group(temp_group))
            temp_group.clear()
    return sum(groups)

file_path = "input.txt"
print(p2_work_lines(get_lines(file_path)))
