| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.71x | 1.69x | 1.63x | 2.36x | 1.94x | 30/30 |
| materialize | 0.97x | 1.96x | 1.56x | 1.00x | 2.15x | 1.36x | 25/30 |
| perft_bulk_d4 | 0.97x | 3.99x | 1.52x | 1.31x | 1.94x | 1.88x | 30/30 |
| perft_full_d3 | 1.07x | 1.62x | 1.69x | 1.41x | 1.93x | 2.12x | 30/30 |
| fen_parse | 1.03x | 1.32x | 1.38x | 1.34x | 1.71x | 1.69x | 30/30 |
| fen_write_native_ep | 1.00x | 4.13x | 8.90x | 1.66x | 27.30x | 2.07x | 30/30 |
| snapshot | 1.07x | 1.69x | 1.13x | 2.53x | 1.17x | 1.01x | 26/30 |
| san_parse_all | 1.08x | 1.96x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 12.51x | N/A | 2.90x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.18x | 1.29x | 5.99x | 1.23x | 1.35x | 30/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.18x | 67.28x | 136.84x | 161.53x | 165.74x | 5/5 |
| chess960_materialize | 1.10x | 1.82x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 3.28x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.90x | 1.87x | 1.11x | 2.26x | 2.29x | 9/10 |
| corpus_materialize | 1.16x | 1.50x | 1.58x | 1.09x | 1.87x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.84x | 2.55x | 1.82x | 2.52x | 1.31x | 10/10 |
| replay_trusted_native | 1.14x | 0.66x | 1.69x | 1.94x | 1.81x | 1.99x | 8/10 |
| replay_checked_native | 1.05x | 2.73x | 1.29x | 9.51x | 8.99x | 10.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
