def main():
    n = int(input())
    a = list(range(1, 2*n+2))
    while True:
        print(a.pop(), flush=True)
        t = int(input())
        if t == 0:
            break
        a.remove(t)


if __name__ == "__main__":
    main()