def main():
    q = int(input())
    a = []
    for _ in range(0, q):
        inp = list(map(lambda x: int(x), input().strip().split(' ')))
        if inp[0] == 1:
            a.append(inp[1])
            a.sort()
            continue
        x = inp[1]
        k = inp[2]
        
        

if __name__ == '__main__':
    main()
