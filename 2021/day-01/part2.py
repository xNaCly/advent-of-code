n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    c = [int(x.strip()) for x in f.readlines()]

r = []
for i in range(len(c)):
    if i+2 >= len(c):
        break
    r.append(sum(c[i:i+3]))

s = 0
for i in range(len(r)):
    if i == 0:
        continue
    if r[i-1] < r[i]:
        s += 1

print(s)
