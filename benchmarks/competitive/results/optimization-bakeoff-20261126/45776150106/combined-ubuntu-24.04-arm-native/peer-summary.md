| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.65x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.39x | 1.21x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.39x | 1.40x | 1.26x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 0.86x | 0.71x | 0.77x | 0.65x | 0.93x | 0.90x | 2/30 |
| fen_parse | 0.96x | 1.20x | 1.28x | 1.23x | 1.71x | 1.36x | 30/30 |
| fen_write_native_ep | 0.98x | 2.94x | 6.05x | 1.02x | 19.72x | 1.60x | 28/30 |
| snapshot | 0.89x | 1.04x | 0.97x | 1.38x | 0.96x | 0.87x | 12/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 10.11x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 0.80x | 0.95x | 2.57x | 0.77x | 0.77x | 8/30 |
| make_unmake_full | 0.81x | N/A | N/A | 0.94x | N/A | N/A | 1/6 |
| startpos | 0.50x | 0.61x | 35.85x | 61.13x | 82.84x | 67.27x | 4/5 |
| chess960_materialize | 0.99x | 1.40x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.30x | 1.17x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 28.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.04x | 19.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.49x | 1.73x | 1.40x | 2.04x | 2.24x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.48x | 1.19x | 1.86x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.94x | 1.39x | 1.90x | 0.97x | 8/10 |
| replay_trusted_native | 0.97x | 0.41x | 1.04x | 1.28x | 1.40x | 1.45x | 8/10 |
| replay_checked_native | 0.93x | 1.71x | 0.84x | 5.63x | 5.02x | 6.51x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
