"""implementing day7 in python"""
import sys

if __name__ == "__main__":
    n = int(input().strip())

    int_array = list(map(int, input().rstrip().split()))

    if len(int_array) < n:
        print("invalid number of values")
        sys.exit()
    if len(int_array) > n:
        diff = len(int_array) - n
        int_array = int_array[:-diff]
    int_array.reverse()
    curr = 0
    reversed_string = ""

    for i in int_array:
        if curr == 0:
            reversed_string += str(i)
        if curr > 0:
            reversed_string += " " + str(i)
        curr += 1

    print(reversed_string)
