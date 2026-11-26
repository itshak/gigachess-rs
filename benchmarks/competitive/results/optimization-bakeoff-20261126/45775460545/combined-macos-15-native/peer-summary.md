| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.27x | 1.30x | 1.69x | 1.60x | 28/30 |
| materialize | 1.04x | 1.99x | 1.51x | 1.11x | 2.49x | 1.34x | 29/30 |
| perft_bulk_d4 | 1.13x | 4.21x | 1.51x | 1.28x | 1.91x | 1.82x | 30/30 |
| perft_full_d3 | 1.15x | 0.91x | 1.01x | 0.83x | 1.20x | 1.21x | 16/30 |
| fen_parse | 1.15x | 1.30x | 1.32x | 1.40x | 1.67x | 1.41x | 30/30 |
| fen_write_native_ep | 1.04x | 3.12x | 6.39x | 1.17x | 19.98x | 1.47x | 29/30 |
| snapshot | 1.09x | 1.61x | 1.09x | 2.31x | 1.09x | 0.98x | 24/30 |
| san_parse_all | 1.33x | 1.92x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.13x | 11.53x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 0.77x | 0.87x | 3.90x | 0.82x | 0.91x | 8/30 |
| make_unmake_full | 1.26x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 0.95x | 1.27x | 71.71x | 169.15x | 180.47x | 146.72x | 5/5 |
| chess960_materialize | 1.31x | 1.58x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.24x | 3.21x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 38.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.15x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 19.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.23x | 5.98x | 1.83x | 1.19x | 2.11x | 2.45x | 10/10 |
| corpus_materialize | 1.20x | 1.63x | 1.53x | 1.12x | 2.01x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.08x | 1.38x | 1.94x | 1.29x | 1.74x | 0.88x | 8/10 |
| replay_trusted_native | 1.05x | 0.44x | 1.20x | 1.14x | 1.17x | 1.26x | 8/10 |
| replay_checked_native | 1.27x | 2.14x | 1.00x | 8.74x | 6.52x | 8.32x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
