| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.07x | 1.54x | 1.67x | 2.00x | 2.00x | 30/30 |
| materialize | 1.02x | 1.87x | 1.49x | 1.28x | 2.07x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.76x | 1.53x | 1.39x | 1.95x | 2.00x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.07x | 1.55x | 1.48x | 29/30 |
| fen_parse | 0.99x | 1.23x | 1.32x | 1.29x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.38x | 9.20x | 1.57x | 29.53x | 2.38x | 30/30 |
| snapshot | 1.00x | 1.21x | 1.03x | 1.61x | 1.05x | 1.02x | 28/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.79x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.11x | 1.31x | 3.57x | 1.07x | 1.09x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.21x | 74.86x | 123.19x | 169.80x | 136.11x | 5/5 |
| chess960_materialize | 1.08x | 1.63x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.76x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 24.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.82x | 1.85x | 1.51x | 2.18x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.52x | 1.21x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.82x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.31x | 1.60x | 1.74x | 1.81x | 8/10 |
| replay_checked_native | 1.00x | 2.05x | 0.99x | 6.64x | 5.94x | 7.69x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
