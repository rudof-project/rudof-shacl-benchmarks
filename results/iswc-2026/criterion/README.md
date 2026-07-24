# Microbenchmarks results

The following results display the mean validation time (in milliseconds) of both rudof versions and the ratio between them.

## ERA

TBD

## ICDD

| Case | `t_v1` (ms) | `t_v2` (ms) | `t_v1` / `t_v2` |
|:------------------:|:------:|:-------:|:-----:|
| `binary-1`         | 0.396  | 1.402   | 0.283 |
| `binary-2`         | 4.381  | 14.217  | 0.308 |
| `binary-3`         | 10.030 | 31.439  | 0.319 |
| `binary-4`         | 51.521 | 148.087 | 0.348 |
| `directed1ton-1`   | 0.325  | 1.241   | 0.262 |
| `directed1ton-2`   | 3.258  | 10.847  | 0.300 |
| `directed1ton-3`   | 7.821  | 24.300  | 0.322 |
| `directed1ton-4`   | 34.884 | 98.581  | 0.354 |
| `directedbinary-1` | 0.367  | 1.407   | 0.261 |
| `directedbinary-2` | 4.338  | 14.080  | 0.308 |
| `directedbinary-3` | 10.206 | 30.072  | 0.339 |
| `directedbinary-4` | 52.136 | 147.453 | 0.354 |

## LUBM


| Case | `t_v1` (ms) | `t_v2` (ms) | `t_v1` / `t_v2` |
|:----------------:|:--------:|:------:|:-------:|
| 5 universities   | 8567.624 | 40.302 | 212.588 |
| 10 universities  | 9707.206 | 44.657 | 217.374 |
| 50 universities  | 9692.994 | 44.671 | 216.985 |
| 100 universities | 9719.700 | 44.838 | 216.774 |
| 500 universities | 9727.000 | 44.654 | 217.831 |

---

> `t_v1` stands for time of rudof v1 (v0.1.146)

> `t_v2` stands for time of rudof v2 (v0.3.7)