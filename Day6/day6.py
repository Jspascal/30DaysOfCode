"""implementing day6"""

if __name__ == "__main__":
    T = int(input())

    for _ in range(0, T):
        indexedString = input()
        char_vector = [*indexedString]
        evenString = ""
        oddString = ""

        for i in range(len(char_vector)):
            if i == 0 or i % 2 == 0:
                evenString += char_vector[i]
            if i % 2 != 0:
                oddString += char_vector[i]

        print(evenString, " ", oddString)
