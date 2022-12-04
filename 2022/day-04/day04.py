with open("input.txt", "r", encoding="utf-8") as f:
    c = [x.strip() for x in f.readlines()]

sum1 = 0
sum2 = 0
for l in c:
    l = [[int(y) for y in x.split("-")] for x in l.split(",")]
    f = set([x for x in range(l[0][0], l[0][1]+1)])
    s = set([x for x in range(l[1][0], l[1][1]+1)])
    r = f & s
    if r == f or r == s:
        sum1 += 1
    if len(r) != 0:
        sum2 += 1


print(f"part1: {sum1}")
print(f"part2: {sum2}")
