import toml
from sys import argv
from glob import glob


def main():
    with open('.cargo/config.toml') as f:
        package = toml.load(f)['package']

    if len(argv) == 2:
        project_id = argv[1]
        cargo_toml_path = f'{project_id}/Cargo.toml'
        code_paths = glob(f'{project_id}/src/bin/*.rs')

        with open(cargo_toml_path, 'r') as f:
            data = toml.load(f)

        with open(cargo_toml_path, 'w') as f:
            data['dependencies'] = package
            toml.dump(data, f)

        for code_path in code_paths:
            with open(code_path) as f:
                code = f.read()
            if not code.split("\n")[0].startswith("use"):
                with open(code_path, 'w') as f:
                    code = '''use proconio::{input,fastout};

#[fastout]
''' + code
                    f.write(code)


if __name__ == "__main__":
    main()
