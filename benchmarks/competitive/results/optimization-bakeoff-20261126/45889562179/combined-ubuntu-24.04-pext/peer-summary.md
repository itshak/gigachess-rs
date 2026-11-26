| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.33x | 1.81x | 1.57x | 2.46x | 1.85x | 30/30 |
| materialize | 1.08x | 2.23x | 1.62x | 1.71x | 2.64x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.71x | 1.48x | 1.16x | 2.29x | 1.80x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.55x | 1.53x | 2.03x | 1.66x | 30/30 |
| fen_parse | 1.01x | 1.50x | 1.40x | 1.18x | 1.64x | 1.16x | 30/30 |
| fen_write_native_ep | 1.01x | 4.94x | 10.69x | 1.95x | 30.28x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.34x | 4.00x | 1.34x | 1.33x | 30/30 |
| san_parse_all | 0.91x | 1.73x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.97x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.12x | 1.36x | 4.05x | 1.32x | 1.70x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 79.04x | 140.25x | 184.18x | 141.63x | 4/5 |
| chess960_materialize | 1.01x | 1.64x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.45x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.84x | 19.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 4.62x | 1.72x | 0.95x | 2.07x | 1.93x | 8/10 |
| corpus_materialize | 0.98x | 1.39x | 1.26x | 1.05x | 1.69x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.11x | 2.28x | 3.04x | 1.55x | 10/10 |
| replay_trusted_native | 1.02x | 0.91x | 1.60x | 1.65x | 2.31x | 2.61x | 9/10 |
| replay_checked_native | 1.00x | 2.23x | 1.14x | 9.20x | 8.98x | 10.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
