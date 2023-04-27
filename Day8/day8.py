"""implementing day8 in python"""

if __name__ == "__main__":
    T = int(input())

    contacts = {}

    for _ in range(0, T):
        contact = list(input().split(" "))
        contact_infos = [(contact[0], contact[1])]
        contacts.update(contact_infos)

    for _ in range(0, T):
        contact_name = input()
        if contacts.get(contact_name):
            print(contacts.get(contact_name))
        else:
            print("Not found")
