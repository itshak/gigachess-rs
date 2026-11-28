| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.69x | 1.83x | 2.20x | 2.19x | 30/30 |
| materialize | 1.01x | 1.97x | 1.54x | 1.36x | 2.17x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.04x | 1.63x | 1.49x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.14x | 1.67x | 1.81x | 1.51x | 2.22x | 2.10x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.28x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.52x | 9.35x | 1.55x | 29.94x | 2.41x | 30/30 |
| snapshot | 0.99x | 1.11x | 1.02x | 1.47x | 1.00x | 0.95x | 19/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.45x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.75x | 127.57x | 169.25x | 135.14x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 25.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.30x | 1.40x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.04x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.81x | 1.86x | 1.51x | 2.19x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.23x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.66x | 2.00x | 2.25x | 2.27x | 8/10 |
| replay_checked_native | 1.02x | 2.34x | 1.15x | 7.70x | 6.94x | 8.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
