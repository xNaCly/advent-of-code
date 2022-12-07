n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    # convert to lines, append an empty line to indicate end
    c = [x.strip() for x in f.readlines()] + ['']

t = []
r = []
for l in c:
    if len(l) == 0:
        gm = len(t)
        k = []
        for i in t:
            k = k + i
        r.append(set([i for i in k if k.count(i) == gm]))
        t.clear()
    else:
        t.append(list(l))
print(sum([len(x) for x in r]))
