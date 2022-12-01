import requests
import sys
import os
from cookie import COOKIE

HOST = "https://adventofcode.com/2022/day"

def parse_args() -> str:
    args = sys.argv # get cli arguments

    print(f":parsing args {args}")

    try:
        args.pop(0) # remove first cli argument (its the filename)
        argument = args[0].split("=")
    except:
        raise ValueError("Not enough arguments")

    if argument[0] == "day":
        return argument[1].strip()
    else:
        raise ValueError("argument must be formated like so: day=<int>")

def create_folder(folder_name: str) -> str:
    if len(folder_name) == 1:
      folder_name = "0"+folder_name
    cwd = os.getcwd()
    target_dir = f"{cwd}/day-{folder_name}"
    print(f":creating folder='{target_dir}'")
    if not os.path.exists(target_dir):
        os.mkdir(target_dir)
    return target_dir

def download_input(day: str) -> str:
    url = f"{HOST}/{day}/input"
    print(f":downloading puzzle input from='{url}'")
    res = requests.get(url, cookies=dict(session=COOKIE))
    return res.text

def write_input_to_file(path: str, content: str) -> bool:
    file_path = f"{path}/input.txt"
    print(f":writing to file='{file_path}'")
    try:
        with open(file=file_path, mode="w", encoding="utf-8") as f:
            f.write(content)
        return True
    except:
        return False


if __name__ == "__main__":
    day = parse_args()
    path = create_folder(day)
    content = download_input(day)
    success = write_input_to_file(path, content)
    if not success:
        raise RuntimeError("couldn't write to file :(")
