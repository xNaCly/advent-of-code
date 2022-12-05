with open("input.txt", mode="r", encoding="utf-8") as f:
    c = [x for x in f.readlines()]

stacks = {}
cmds = []
spaces = []
amount_of_stacks = 9
get_stack_by_char_pos: dict[int, int] = {
        1: 1
        } 

# creates the stacks
for i in range(1, amount_of_stacks+1):
    stacks[f"stack-{i}"] = []

# loops over input lines, but onl the ones starting with '1'
for l in c:
    if l.strip().startswith("1"):
        # loop over all to integers converted characters
        for i in [int(x) for x in l.split()]:
            # create a lookup table to later insert the characters in the correct stack
            get_stack_by_char_pos[int(l.index(str(i)))] = i

# loop over all lines again
for l in c:
    # only process lines starting with 'm' (move instructions)
    if l.startswith("m"):
        # convert instructions to an array
        l = l.split()
        cmds.append([int(x) for x in [l[1], l[3], l[5]]])
    # ignore empty lines or the line we processed before
    elif len(l.strip()) == 0 or l.strip().startswith("1"):
        continue
    # process crate lines
    else:
        # format string for improved processing, split string
        e = l.strip().replace("[", "").replace("]", "").split()
        # loop over crates
        for i in e:
            pos = l.index(i)
            # replace already indexed crates
            l = l[:pos] + " " + l[pos+1:]
            # use index in array to lookup corresponding crate 
            stacks[f"stack-{get_stack_by_char_pos[pos]}"].append(i)


# reverse item order in stacks to fit the stack criteria
for s in stacks:
    stacks[s] = stacks[s][::-1]

# execute instructions
for e in cmds:
    # get crates which will be moved
    el = stacks[f"stack-{e[1]}"][-e[0]:]
    # cache previous stack state
    prev_1 = stacks[f"stack-{e[1]}"]
    # remove moved crates
    stacks[f"stack-{e[1]}"] = prev_1[:-e[0]]
    # cache previous stack state
    prev_2 = stacks[f"stack-{e[2]}"]
    # merge old and new crates in stack
    stacks[f"stack-{e[2]}"] = prev_2 + el

# get topmost elements and merge them together in a string
res = ""
for s in stacks:
    res += stacks[s].pop()
print(res)
