| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.50x | 1.47x | 1.36x | 2.04x | 1.77x | 30/30 |
| materialize | 0.98x | 2.07x | 1.36x | 1.18x | 2.28x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.82x | 1.44x | 1.19x | 2.21x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 0.97x | 1.11x | 0.86x | 1.45x | 1.62x | 21/30 |
| fen_parse | 0.92x | 1.17x | 1.34x | 1.30x | 1.85x | 1.32x | 29/30 |
| fen_write_native_ep | 0.97x | 3.70x | 7.01x | 1.51x | 31.15x | 2.06x | 30/30 |
| snapshot | 0.75x | 1.26x | 0.77x | 4.11x | 0.78x | 0.75x | 12/30 |
| san_parse_all | 0.96x | 1.94x | N/A | 4.56x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.64x | N/A | 3.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.13x | 1.81x | 1.42x | 7.65x | 1.17x | 1.90x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 93.14x | 173.13x | 242.74x | 188.28x | 5/5 |
| chess960_materialize | 1.05x | 1.47x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.69x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 37.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.54x | 24.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.87x | 5.51x | 1.62x | 1.08x | 2.13x | 2.06x | 9/10 |
| corpus_materialize | 0.94x | 1.70x | 1.27x | 1.07x | 2.22x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.61x | 1.94x | 1.67x | 1.93x | 1.36x | 10/10 |
| replay_trusted_native | 0.99x | 0.71x | 2.46x | 1.41x | 2.14x | 2.59x | 8/10 |
| replay_checked_native | 1.01x | 2.11x | 0.93x | 6.99x | 6.87x | 9.01x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
