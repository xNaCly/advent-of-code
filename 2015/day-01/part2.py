n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    c = f.read().strip()

cur_floor = 0
for i, c in enumerate(c):
    if c == "(":
        cur_floor += 1
    elif c == ")":
        cur_floor -= 1

    if cur_floor == -1:
        print(i+1)
        break

