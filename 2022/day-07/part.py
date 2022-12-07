filename = "input.txt"

# that was hard as fuck, i could'nt wrap my head around the recursion, solution gotten from:
# https://www.reddit.com/r/adventofcode/comments/zesk40/comment/iz8o4ty/
# this day was the hardest for me, i guess i gotta look into recursion and step up my game

with open(filename, "r", encoding="utf-8") as f:
    c = [x.strip() for x in f.readlines()]

directory = {'/' : {}}
pwd = []

for l in c:
    l = l.split(' ')
    if l[0] == '$':
        if l[1] == 'cd':
            match l[2]:
                case '..':
                    pwd.pop()
                case '/':
                    pwd = [directory['/']]
                case _:
                    pwd.append(pwd[-1][l[2]])
    else:
        pwd[-1][l[1]] = {} if l[0] == 'dir' else int(l[0])

fs = {}

def rec_size(bd, dir_name):
    r = 0
    for d, subdir in bd.items():
        if type(subdir) == dict:
            key = dir_name + '/' + d if dir_name else '/'
            fs[key] = rec_size(subdir, key)
            r += fs[key]
        else:
            r += subdir
    return r

rec_size(directory, '')

print(sum(f for f in fs.values() if f <= 100_000))

print(min(f for f in fs.values() if f > fs['/'] - 40_000_000))
