with open("input.txt", mode="r", encoding="utf-8") as f:
    c = [x for x in f.readlines()]

stacks = {}
cmds = []
spaces = []
amount_of_stacks = 9
get_stack_by_char_pos: dict[int, int] = {
        1: 1
        } 

for i in range(1, amount_of_stacks+1):
    stacks[f"stack-{i}"] = []

for l in c:
    if l.strip().startswith("1"):
        for i in [int(x) for x in l.split()]:
            get_stack_by_char_pos[int(l.index(str(i)))] = i

print(get_stack_by_char_pos)
for l in c:
    if l.startswith("m"):
        l = l.split()
        cmds.append([int(x) for x in [l[1], l[3], l[5]]])
    elif len(l.strip()) == 0 or l.strip().startswith("1"):
        continue
    else:
        e = l.strip().replace("[", "").replace("]", "").split()
        for i in e:
            pos = l.index(i)
            l = l[:pos] + " " + l[pos+1:]
            stacks[f"stack-{get_stack_by_char_pos[pos]}"].append(i)


for s in stacks:
    stacks[s] = stacks[s][::-1]


for e in cmds:
    for i in range(0, e[0]):
        el = stacks[f"stack-{e[1]}"].pop()
        stacks[f"stack-{e[2]}"].append(el)

res = ""
for s in stacks:
    res += stacks[s].pop()
print(res)
