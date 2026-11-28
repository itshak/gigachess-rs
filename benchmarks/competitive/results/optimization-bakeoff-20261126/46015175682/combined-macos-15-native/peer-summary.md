| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 4.94x | 1.69x | 1.76x | 2.23x | 2.00x | 30/30 |
| materialize | 1.03x | 1.98x | 1.57x | 1.14x | 2.30x | 1.46x | 29/30 |
| perft_bulk_d4 | 1.09x | 4.48x | 1.61x | 1.35x | 2.00x | 1.98x | 29/30 |
| perft_full_d3 | 1.17x | 1.83x | 1.80x | 1.43x | 2.07x | 2.10x | 30/30 |
| fen_parse | 1.00x | 1.25x | 1.39x | 1.30x | 1.58x | 1.47x | 30/30 |
| fen_write_native_ep | 1.07x | 4.16x | 8.86x | 1.73x | 26.81x | 2.29x | 30/30 |
| snapshot | 0.98x | 1.71x | 1.18x | 2.53x | 1.21x | 0.98x | 26/30 |
| san_parse_all | 0.96x | 1.77x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 12.40x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.17x | 1.24x | 5.66x | 1.13x | 1.26x | 29/30 |
| make_unmake_full | 1.08x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.35x | 72.04x | 154.49x | 177.75x | 150.16x | 5/5 |
| chess960_materialize | 1.05x | 1.70x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.13x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 36.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.52x | 1.47x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.51x | 2.07x | 1.23x | 2.38x | 2.70x | 10/10 |
| corpus_materialize | 1.00x | 1.59x | 1.48x | 1.18x | 2.00x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.80x | 2.78x | 2.01x | 2.60x | 1.41x | 10/10 |
| replay_trusted_native | 1.02x | 0.63x | 1.55x | 1.65x | 1.95x | 1.89x | 8/10 |
| replay_checked_native | 1.00x | 2.72x | 1.27x | 9.65x | 8.61x | 10.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
