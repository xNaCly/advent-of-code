n = "i.txt"

with open(n, mode="r", encoding="utf-8") as f:
    c = [[int(y) for y in x.strip().split("x")] for x in f.readlines()]

r = 0
for i in c:
    l, w, h = i
    s = [l * w, w * h, h * l]
    s.sort()
    r += sum([x*2 for x in s]) + s[0]
print(r)
