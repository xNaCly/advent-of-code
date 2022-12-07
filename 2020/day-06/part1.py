n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    # convert to lines, append an empty line to indicate end
    c = [x.strip() for x in f.readlines()] + ['']

t = []
r = []
for l in c:
    if len(l) == 0:
        r.append(set(t))
        t.clear()
    else:
        t = t + list(l)

print(sum([len(x) for x in r]))
