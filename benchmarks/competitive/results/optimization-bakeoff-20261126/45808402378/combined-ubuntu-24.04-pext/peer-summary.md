| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.61x | 1.60x | 1.62x | 2.51x | 1.83x | 30/30 |
| materialize | 0.98x | 2.09x | 1.68x | 1.19x | 2.14x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.98x | 1.54x | 1.29x | 2.48x | 2.02x | 30/30 |
| perft_full_d3 | 1.11x | 1.24x | 1.26x | 0.97x | 1.83x | 1.46x | 26/30 |
| fen_parse | 1.01x | 1.47x | 1.65x | 1.20x | 2.17x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 4.17x | 9.95x | 1.85x | 27.85x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.73x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.31x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.28x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.79x | 1.17x | 4.39x | 1.36x | 1.53x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 110.52x | 134.05x | 217.10x | 143.71x | 5/5 |
| chess960_materialize | 1.02x | 1.81x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.03x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 15.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 4.28x | 1.64x | 1.27x | 2.29x | 1.88x | 9/10 |
| corpus_materialize | 0.96x | 1.85x | 1.69x | 1.21x | 2.78x | 1.46x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.29x | 3.12x | 2.28x | 2.96x | 1.61x | 10/10 |
| replay_trusted_native | 0.99x | 0.80x | 1.40x | 1.55x | 2.69x | 3.13x | 9/10 |
| replay_checked_native | 0.99x | 2.52x | 0.98x | 5.52x | 6.38x | 7.25x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
