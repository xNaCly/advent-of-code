def get_lines(file_path: str) -> list[str]:
    with open(file=file_path, mode="r", encoding="utf-8") as f:
        return [l for l in map(lambda x: x.strip(), f.readlines())]
    

priority_lookup = [ "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z" ]


def generate_occurences_hash_map(half_of_line, hashmap):
    for c in half_of_line:
        if c in hashmap:
            hashmap[c] = hashmap[c] + 1
        else:
            hashmap[c] = 1

def work_lines(lines: list[str]) -> int:
    prio_sum = 0
    for l in lines:
        first_half_map = {}
        second_half_map = {}

        # split l in the middle using the index slice method
        l = [l[:int(len(l)/2)], l[int(len(l)/2):]] 

        # generate hashmaps with the counter for each character
        generate_occurences_hash_map(l[0], first_half_map)
        generate_occurences_hash_map(l[1], second_half_map)

        matching_key = ""
        for key in first_half_map.keys():
            if key in second_half_map.keys():
                matching_key = key
            else:
                continue

        prio_sum += priority_lookup.index(matching_key)+1

    return prio_sum

# file_path = "input.txt"
# print(work_lines(get_lines(file_path)))
