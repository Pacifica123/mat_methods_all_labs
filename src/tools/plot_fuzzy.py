# tools/plot_fuzzy.py
import sys, json
import numpy as np
import matplotlib.pyplot as plt

def mu_true(x, a1):
    if x < a1 or x > 1.0:
        # formula will produce <0 ; clamp later
        val = 0.5*(1 + (np.pi/2)*( (2*x -1 - a1)/(1 - a1) ))
    else:
        val = 0.5*(1 + (np.pi/2)*( (2*x -1 - a1)/(1 - a1) ))
    return max(0.0, min(1.0, val))

def mu_false(x, a1):
    if x < 0.0 or x > 1.0 - a1:
        val = 0.5*(1 + (np.pi/2)*( (1 - a1 - 2*x)/(1 - a1) ))
    else:
        val = 0.5*(1 + (np.pi/2)*( (1 - a1 - 2*x)/(1 - a1) ))
    return max(0.0, min(1.0, val))

def main(json_path):
    with open(json_path,'r') as f:
        data = json.load(f)

    r = data['r']
    a1 = data['a1']

    xs = np.linspace(0,1,501)
    mu_t = [mu_true(x,a1) for x in xs]
    mu_f = [mu_false(x,a1) for x in xs]
    
    # фактчек
    u = np.linspace(0, 1, 200)
    sum_check = [mu_true(x, a1) + mu_false(1-x, a1)
                for x in u if mu_true(x,a1) is not None and mu_false(1-x,a1) is not None]
    print("Среднее значение суммы:", np.mean(sum_check))

    fig, ax = plt.subplots(figsize=(8,4))
    ax.plot(xs, mu_t, label='mu_true(x)')
    ax.plot(xs, mu_f, label='mu_false(x)')
    ax.fill_between(xs, 0, 1, where=(xs>=r['low']) & (xs<=r['high']), color='gray', alpha=0.15)
    ax.axvline(r['low'], linestyle='--', label=f"r_low={r['low']:.3f}")
    ax.axvline(r['high'], linestyle='--', label=f"r_high={r['high']:.3f}")

    # annotate membership values at endpoints
    mu_t_low = mu_true(r['low'], a1)
    mu_t_high = mu_true(r['high'], a1)
    mu_f_low = mu_false(r['low'], a1)
    mu_f_high = mu_false(r['high'], a1)

    ax.scatter([r['low'], r['high']],[mu_t_low, mu_t_high], color='tab:blue')
    ax.scatter([r['low'], r['high']],[mu_f_low, mu_f_high], color='tab:orange')
    ax.text(r['low'], mu_t_low+0.03, f"{mu_t_low:.3f}", color='tab:blue')
    ax.text(r['high'], mu_t_high+0.03, f"{mu_t_high:.3f}", color='tab:blue')
    ax.text(r['low'], mu_f_low-0.06, f"{mu_f_low:.3f}", color='tab:orange')
    ax.text(r['high'], mu_f_high-0.06, f"{mu_f_high:.3f}", color='tab:orange')

    ax.set_xlim(0,1)
    ax.set_ylim(-0.05,1.05)
    ax.set_xlabel('x (degree of truth)')
    ax.set_ylabel('membership')
    ax.set_title('Лингвистические множества: Истина / Ложь')
    ax.legend()
    plt.tight_layout()
    plt.savefig('./data/fuzzy_plot.png')
    plt.close()
    print('Saved fuzzy_plot.png')

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: python plot_fuzzy.py path/to/answer_for_fuzzy.json')
        sys.exit(1)
    main(sys.argv[1])
