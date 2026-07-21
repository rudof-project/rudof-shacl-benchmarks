import sys, os
import icdd, lubm, era

def __get_files(input: str) -> list[str]:
    if not os.path.isdir(input): return [input]

    output = list()
    for path, dirs, files in os.walk(input):
        for f in files:
            output.append(os.path.join(path, f))

    return output


def main() -> None:
    filename = sys.argv[1]
    dataset = sys.argv[2]
    mode = sys.argv[3]
    other = sys.argv[4:]

    print(f"[+] Patching {filename}")
    print(f"[+] With {dataset} ({mode}) patcher")

    for f in __get_files(filename):
        print(f"[+] Patching {f}...")
        if dataset.lower() == "icdd":
            icdd.patch(mode, f, other)
        elif dataset.lower() == "lubm":
            lubm.patch(mode, f, other)
        elif dataset.lower() == "era":
            era.patch(mode, f, other)
        else:
            raise Exception(f"Dataset ({dataset}) not valid")

if __name__ == "__main__":
    main()
