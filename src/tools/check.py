import numpy as np
import matplotlib.pyplot as plt
from math import pi

# параметры
a1 = 0.2
p = np.array([0.6, 0.8])
q = np.array([0.3, 0.9])
r = np.minimum(p, q)  # [0.3, 0.8]

# сетка
u = np.linspace(0, 1, 501)

# формулы из задания (возвращаем None вне области определения)
def mu_true_form(x, a1):
    if x < a1 or x > 1.0:
        return None
    v = 0.5 * (1.0 + (pi/2.0) * ((2.0*x - 1.0 - a1) / (1.0 - a1)))
    return max(0.0, min(1.0, v))

def mu_false_form(x, a1):
    if x < 0.0 or x > 1.0 - a1:
        return None
    v = 0.5 * (1.0 + (pi/2.0) * ((1.0 - a1 - 2.0*x) / (1.0 - a1)))
    return max(0.0, min(1.0, v))

mu_t_form = np.array([mu_true_form(x, a1) if mu_true_form(x,a1) is not None else np.nan for x in u])
mu_f_form = np.array([mu_false_form(x, a1) if mu_false_form(x,a1) is not None else np.nan for x in u])

# линейная упрощенная версия друга
mu_t_lin = np.clip(2*u - 1, 0, 1)
mu_f_lin = 1 - mu_t_lin

# plotting
plt.figure(figsize=(10,5))

# формулы задания
plt.subplot(1,2,1)
plt.plot(u, mu_t_form, label='mu_true (formula)', linewidth=2)
plt.plot(u, mu_f_form, label='mu_false (formula)', linewidth=2)
plt.fill_between(u, 0, 1, where=(u>=r[0]) & (u<=r[1]), color='lightgreen', alpha=0.25)
plt.title('По формуле (задание)')
plt.xlabel('x'); plt.ylim(-0.05,1.05); plt.grid(True); plt.legend()

# линейная версия друга
plt.subplot(1,2,2)
plt.plot(u, mu_t_lin, label='mu_true (linear)', linewidth=2)
plt.plot(u, mu_f_lin, label='mu_false (linear)', linewidth=2)
plt.fill_between(u, 0, 1, where=(u>=r[0]) & (u<=r[1]), color='lightgreen', alpha=0.25)
plt.title('Линейная аппроксимация (друг)')
plt.xlabel('x'); plt.ylim(-0.05,1.05); plt.grid(True); plt.legend()

plt.tight_layout()
plt.show()
