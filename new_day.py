import requests
import sys
import os
from cookie import COOKIE

HOST = "https://adventofcode.com"

def parse_args() -> str:
    """
    python3 new_day.py i=2022/2
    """
    args = sys.argv # get cli arguments

    print(f":parsing args {args}")

    try:
        args.pop(0) # remove first cli argument (its the filename)
        argument = args[0].split("=")
    except:
        raise ValueError("Not enough arguments")

    if argument[0] == "i":
        return argument[1].strip()
    else:
        raise ValueError("argument must be formated like so: i=<year>/<day>")

def create_folder(year: str, day: str) -> str:
    if len(day) == 1:
      day = "0"+day
    cwd = os.getcwd()
    target_dir = f"{cwd}/{year}/day-{day}"
    print(f":creating folder='{target_dir}'")
    if not os.path.exists(target_dir):
        os.makedirs(target_dir)
    return target_dir

def download_input(year: str, day: str) -> str:
    url = f"{HOST}/{year}/day/{day}/input"
    print(f":downloading puzzle input from='{url}'")
    res = requests.get(url, cookies=dict(session=COOKIE))
    return res.text

def write_input_to_file(path: str, content: str) -> bool:
    file_path = f"{path}/i.txt"
    print(f":writing to file='{file_path}'")
    try:
        with open(file=file_path, mode="w", encoding="utf-8") as f:
            f.write(content)
        return True
    except:
        return False


if __name__ == "__main__":
    year, day = parse_args().split("/")
    path = create_folder(year, day)
    content = download_input(year, day)
    success = write_input_to_file(path, content)
    if not success:
        raise RuntimeError("couldn't write to file :(")

