| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.58x | 1.59x | 1.62x | 2.56x | 1.83x | 30/30 |
| materialize | 1.00x | 2.12x | 1.66x | 1.15x | 2.21x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.02x | 1.55x | 1.28x | 2.44x | 2.02x | 30/30 |
| perft_full_d3 | 0.99x | 1.25x | 1.28x | 0.98x | 1.77x | 1.84x | 26/30 |
| fen_parse | 1.09x | 1.40x | 1.46x | 1.17x | 2.05x | 1.31x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.58x | 1.81x | 27.03x | 2.32x | 30/30 |
| snapshot | 0.99x | 1.89x | 1.25x | 3.27x | 2.77x | 1.25x | 30/30 |
| san_parse_all | 0.97x | 2.18x | N/A | 3.79x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.99x | N/A | 2.04x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.86x | 1.23x | 4.41x | 1.34x | 1.61x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.20x | 93.60x | 150.79x | 224.26x | 159.39x | 5/5 |
| chess960_materialize | 1.02x | 1.84x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.92x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 20.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.14x | 17.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.24x | 1.64x | 1.26x | 2.28x | 1.87x | 9/10 |
| corpus_materialize | 0.99x | 1.88x | 1.76x | 1.23x | 2.83x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.31x | 3.12x | 2.35x | 3.01x | 1.66x | 10/10 |
| replay_trusted_native | 1.04x | 1.12x | 1.95x | 2.31x | 3.91x | 4.51x | 10/10 |
| replay_checked_native | 1.00x | 2.67x | 1.02x | 5.89x | 6.69x | 7.92x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
