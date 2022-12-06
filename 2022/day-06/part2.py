file_name = "input.txt"
with open(file_name, mode="r", encoding="utf-8") as f:
    c = [x for x in f.read().strip()]

for i in range(0, len(c)):
    if i > 12:
        l = [
                c[i-13],
                c[i-12],
                c[i-11],
                c[i-10],
                c[i-9],
                c[i-8],
                c[i-7],
                c[i-6],
                c[i-5],
                c[i-4],
                c[i-3],
                c[i-2],
                c[i-1],
            ]
        if c[i] not in l and len(set(l)) == len(l):
            print(i+1)
            break
