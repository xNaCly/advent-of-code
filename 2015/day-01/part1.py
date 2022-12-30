n = "i.txt"

with open(n, "r", encoding="utf-8") as f:
    c = f.read().strip()
    print(c.count("(") - c.count(")"))
