| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.62x | 1.58x | 1.61x | 2.44x | 1.86x | 30/30 |
| materialize | 1.01x | 1.96x | 1.63x | 1.26x | 2.23x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.99x | 1.54x | 1.34x | 2.38x | 1.87x | 30/30 |
| perft_full_d3 | 0.76x | 0.97x | 1.08x | 0.96x | 1.37x | 1.29x | 19/30 |
| fen_parse | 0.85x | 1.17x | 1.20x | 1.05x | 1.90x | 1.14x | 28/30 |
| fen_write_native_ep | 1.00x | 2.28x | 5.37x | 0.92x | 15.32x | 1.09x | 24/30 |
| snapshot | 1.33x | 1.60x | 1.20x | 2.73x | 1.19x | 1.04x | 30/30 |
| san_parse_all | 1.02x | 2.14x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 10.47x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.97x | 1.13x | 3.82x | 1.22x | 1.40x | 29/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.19x | 72.79x | 132.74x | 206.95x | 134.96x | 5/5 |
| chess960_materialize | 1.00x | 1.49x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.70x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 24.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 19.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.95x | 1.90x | 1.47x | 2.79x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.71x | 1.59x | 1.24x | 2.20x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.48x | 3.21x | 2.46x | 3.12x | 1.81x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.41x | 1.60x | 2.12x | 2.37x | 8/10 |
| replay_checked_native | 0.98x | 2.27x | 1.00x | 7.68x | 7.41x | 8.79x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
