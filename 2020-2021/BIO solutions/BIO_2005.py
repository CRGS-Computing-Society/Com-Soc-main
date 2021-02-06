from math import gcd

def solve_1():
    decimal_str = input()
    mulitplier = len(decimal_str) - 2
    frac = (float(decimal_str) * 10**mulitplier, 10**mulitplier)
    gcf = gcd(int(frac[0]), int(frac[1]))
    frac = (frac[0]/gcf, frac[1]/gcf)
    frac = (int(frac[0]), int(frac[0]))
    print(f"{frac[0]} / {frac[1]}")

def solve_3():

    def solve_rec(actors):
        if len(actors) == 1:
            return 1

        allowed = True
        for i in range(len(actors)-1):
            if actors[i] < actors[i+1]:
                allowed = False
        if not allowed:
            return 0

        if actors[-1] == 0:
            del actors[-1]

        total = 0
        for i in range(len(actors)):
            new_actors = actors.copy()
            new_actors[i] -= 1
            total += solve_rec(new_actors)
        
        return total

    no_actors = int(input())
    actors = [int(i) for i in input().split(" ")]
    assert no_actors == len(actors)

    print(solve_rec(actors))

solve_1()
solve_3()
