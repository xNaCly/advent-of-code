import requests
import sys
import os
from cookie import COOKIE

HOST = "https://adventofcode.com/2022/day"

def parse_args() -> str:
    print(":parsing args")
    args = sys.argv # get cli arguments
    args.pop(0) # remove first cli argument (its the filename)
    argument = args[0].split("=")
    if argument[0] == "day":
        return argument[1].strip()
    else:
        raise ValueError("argument must be formated like so: day=<int>")

def create_folder(folder_name: str) -> str:
    if len(folder_name) == 1:
      folder_name = "0"+folder_name
    print(":creating folder...")
    cwd = os.getcwd()
    target_dir = f"{cwd}/day-{folder_name}"
    if not os.path.exists(target_dir):
        os.mkdir(target_dir)
    return target_dir

def download_input(day: str) -> str:
    print(":downloading input")
    res = requests.get(f"{HOST}/{day}/input", cookies=dict(session=COOKIE))
    return res.text

def write_input_to_file(path: str, content: str) -> bool:
    print(":writing to file")
    try:
        with open(f"{path}/input.txt", mode="w", encoding="utf-8") as f:
            f.write(content)
        print("+wrote to file")
        return True
    except:
        print("!writing to file failed")
        return False


if __name__ == "__main__":
    day = parse_args()
    print("day: {day}")
    path = create_folder(day)
    if path:
        print("+created folder.")
    content = download_input(day)
    print("+downloaded input")
    success = write_input_to_file(path, content)
    if not success:
        raise RuntimeError("couldn't write to file :(")
