"""day 3 python implementation : conditional structure"""

import sys


number = int(input())
if number > 100:
    print("Number should be below 100")
    sys.exit()

if number % 2 != 0:
    print("Weird")

if number % 2 == 0:
    if number in range(2, 6):
        print("Not Weird")
    if number in range(6, 21):
        print("Weird")
    if number > 20:
        print("Not Weird")
