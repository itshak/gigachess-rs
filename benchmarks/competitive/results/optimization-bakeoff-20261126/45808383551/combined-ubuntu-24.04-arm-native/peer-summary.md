| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.89x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.83x | 1.44x | 1.26x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.44x | 1.41x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 0.99x | 1.15x | 1.25x | 1.06x | 1.52x | 1.45x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.34x | 1.28x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.39x | 1.58x | 30.03x | 2.40x | 30/30 |
| snapshot | 0.98x | 1.19x | 0.99x | 1.58x | 0.98x | 1.03x | 22/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.67x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.21x | 1.41x | 3.90x | 1.15x | 1.16x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.17x | 71.34x | 121.89x | 158.26x | 127.02x | 5/5 |
| chess960_materialize | 1.05x | 1.59x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.68x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 3.93x | 1.51x | 1.20x | 1.80x | 1.96x | 10/10 |
| corpus_materialize | 0.97x | 1.51x | 1.45x | 1.17x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.90x | 2.09x | 2.81x | 1.45x | 10/10 |
| replay_trusted_native | 1.02x | 0.53x | 1.41x | 1.71x | 1.87x | 1.94x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 0.98x | 6.61x | 5.91x | 7.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
