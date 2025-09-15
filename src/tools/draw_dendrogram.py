import sys
import numpy as np
import matplotlib.pyplot as plt
from scipy.cluster.hierarchy import dendrogram

def main(path_txt: str):
    # Загружаем linkage-матрицу из файла
    # Формат строк: left right dist size
    linkage_matrix = np.loadtxt(path_txt)

    # Рисуем дендрограмму
    fig, ax = plt.subplots(figsize=(8, 5))
    dendrogram(linkage_matrix, ax=ax, labels=None, orientation="top")

    ax.set_title("Dendrogram (Single Linkage)")
    ax.set_xlabel("Objects")
    ax.set_ylabel("Distance")

    plt.tight_layout()
    plt.savefig("../../data/dendrogram.png")
    print("Saved dendrogram.png")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python draw_dendrogram.py answer_for_clustering.txt")
        sys.exit(1)
    main(sys.argv[1])
