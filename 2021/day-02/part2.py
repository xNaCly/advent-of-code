n = "i.txt"

with open(n, "r") as f:
    c = [x.strip() for x in f.readlines()]

h = 0
d = 0
a = 0
for l in c:
    i,v = l.split(" ")
    i = i[0]
    v = int(v)
    match i:
        case "f":
            h += v
            d += a * v
        case "d":
            a += v
        case "u":
            a -= v

print(h*d)
