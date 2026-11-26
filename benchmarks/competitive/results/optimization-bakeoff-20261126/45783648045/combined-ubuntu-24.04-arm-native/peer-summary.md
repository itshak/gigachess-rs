| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 0.99x | 1.81x | 1.44x | 1.23x | 1.99x | 1.37x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.52x | 1.39x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.15x | 1.25x | 1.05x | 1.51x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.35x | 1.28x | 1.79x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 3.01x | 6.15x | 1.03x | 19.55x | 1.58x | 28/30 |
| snapshot | 0.95x | 1.14x | 0.98x | 1.44x | 0.97x | 0.90x | 12/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.78x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.08x | 1.28x | 3.47x | 1.04x | 1.04x | 26/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.49x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.18x | 70.31x | 121.98x | 164.05x | 135.47x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 24.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.80x | 1.83x | 1.49x | 2.16x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.49x | 1.20x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.11x | 2.86x | 1.49x | 10/10 |
| replay_trusted_native | 1.03x | 0.52x | 1.38x | 1.64x | 1.84x | 1.90x | 8/10 |
| replay_checked_native | 1.05x | 2.13x | 1.03x | 6.99x | 6.23x | 8.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
