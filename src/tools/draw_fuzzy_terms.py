import json
import matplotlib.pyplot as plt

with open("data/fuzzy_result.json", "r") as f:
    data = json.load(f)

U = data["universal_set"]
terms = data["terms"]

for term in terms:
    plt.plot(U, term["membership"], label=term["name"])

plt.xlabel("Рост мужчины, см")
plt.ylabel("Степень принадлежности")
plt.title("Функции принадлежности термов")
plt.legend()
plt.grid(True)
plt.savefig("data/fuzzy_terms.png")
print("Saved fuzzy_terms.png")
