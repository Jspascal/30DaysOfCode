"""day 4 implementation"""


class Person:
    def __init__(self, age):
        if age < 0:
            print("Age is not valid, setting age to 0.")
            self.age = 0

        self.age = age

    def amIOld(self):
        match self.age:
            case age if age in range(0, 13):
                print("You are young.")
            case age if age in range(13, 18):
                print("You are a teenager.")
            case age if age > 17:
                print("You are old")

    def yearPasses(self):
        self.age += 1


if __name__ == "__main__":
    T = int(input())

    for _ in range(0, T):
        age = int(input())
        person = Person(age)

        person.amIOld()

        for _ in range(0, 3):
            person.yearPasses()

        person.amIOld()
        print("")
