odds_table = [
    {
        "name": "Cherry",
        "odds": 128,  # 0-127 7-bits
        "payout": 3,
    },
    {
        "name": "Bar",
        "odds": 70,   # 128-191 7-bits
        "payout": 5,
    },
    {
        "name": "DoubleBar",
        "odds": 26,   # 192-217 5-bits
        "payout": 10,
    },
    {
        "name": "TripleBar",
        "odds": 22,   # 218-239 5-bits
        "payout": 15,
    },
    {
        "name": "Seven",
        "odds": 9,    # 240-248 4-bits
        "payout": 20,
    },
    {
        "name": "MinorJackpot",
        "odds": 4,    # 249-252 2-bits
        "payout": 25,
    },
    {
        "name": "MajorJackpot",
        "odds": 2,    # 253-254 2-bits
        "payout": 50,
    },
    {
        "name": "GrandJackpot",
        "odds": 1,    # 255     1-bit
        "payout": 100,
    }
]

for o in odds_table:
    o["probability"] = (o["odds"] / 256.0) ** 3
    o["avg_payout"] = o["probability"] * o["payout"]

print(odds_table)

print("Avg payout: " + str(sum(map(lambda o: o["avg_payout"], odds_table))))
