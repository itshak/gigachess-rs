| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.58x | 1.34x | 2.16x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.57x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.59x | 1.33x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.32x | 1.27x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.58x | 9.53x | 1.59x | 29.90x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.16x | 1.00x | 1.49x | 1.00x | 0.94x | 20/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.40x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.18x | 1.39x | 3.79x | 1.14x | 1.14x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.36x | 127.90x | 169.73x | 137.09x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.00x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.42x | 1.69x | 1.37x | 1.98x | 2.19x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.23x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.11x | 2.93x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.94x | 0.58x | 1.55x | 1.87x | 2.13x | 2.14x | 8/10 |
| replay_checked_native | 0.94x | 2.18x | 1.05x | 7.10x | 6.39x | 8.20x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
