from glob import glob
from os import remove


def main():
    print("Project id: ", end="")
    id = input()
    code_paths = glob(f'{id}/src/bin/*.rs')
    for code_path in code_paths:
        with open(code_path) as f:
            code = f.read()
            if "unimplemented!();" in code:
                remove(code_path)


if __name__ == "__main__":
    main()
