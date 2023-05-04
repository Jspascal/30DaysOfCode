"""Day 10 implementation"""


def convert_to_binary(numb: int) -> list:
    bin_number = []
    while numb > 1:
        i = numb % 2
        bin_number.append(i)
        numb //= 2
        if numb == 1:
            bin_number.append(numb)
    return bin_number


def longest_streak(bi_number: list) -> int:
    tmp = streak = 0
    for i in range(0, len(bi_number)):
        if bi_number[i] == 1:
            tmp += 1
        if bi_number[i] == 0 and tmp >= streak:
            streak = tmp
            tmp = 0
        if bi_number[i] == 0 and tmp < streak:
            tmp = 0
    if tmp > streak:
        streak = tmp
    return streak


if __name__ == "__main__":
    number = int(input())
    binary_number = convert_to_binary(number)
    lng_streak = longest_streak(binary_number)
    print(lng_streak)
