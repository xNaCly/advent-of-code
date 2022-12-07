n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    c = [int(x.strip()) for x in f.readlines()]

r = 0
for i in range(len(c)):
    if i == 0:
        continue
    l = c[i]
    if l > c[i-1]:
        r += 1

print(r)
