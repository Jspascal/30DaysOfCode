"""Day 2"""


def solve(m_cost, tip_perc, tax_perc):
    """function returning the total price of a meal (tip and taxes)"""
    tax = (m_cost * tax_perc) / 100
    tip = (m_cost * tip_perc) / 100
    return m_cost + tax + tip


if __name__ == "__main__":
    meal_cost = float(input().strip())
    tip_percent = float(input().strip())
    tax_percent = float(input().strip())

    total_cost = solve(meal_cost, tip_percent, tax_percent)
    print(round(total_cost))
