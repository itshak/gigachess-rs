| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 4.58x | 1.66x | 1.66x | 2.14x | 1.94x | 30/30 |
| materialize | 1.18x | 2.06x | 1.69x | 1.11x | 2.43x | 1.58x | 29/30 |
| perft_bulk_d4 | 0.93x | 3.44x | 1.27x | 1.08x | 1.58x | 1.59x | 28/30 |
| perft_full_d3 | 1.09x | 1.54x | 1.80x | 1.41x | 2.02x | 1.96x | 30/30 |
| fen_parse | 0.95x | 1.22x | 1.18x | 1.26x | 1.48x | 1.32x | 28/30 |
| fen_write_native_ep | 1.17x | 4.44x | 9.03x | 1.83x | 29.47x | 2.32x | 30/30 |
| snapshot | 1.12x | 1.64x | 1.08x | 2.37x | 1.08x | 0.94x | 25/30 |
| san_parse_all | 0.97x | 1.77x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.14x | 10.66x | N/A | 2.44x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.02x | 1.17x | 5.75x | 1.18x | 1.25x | 26/30 |
| make_unmake_full | 1.14x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.54x | 1.24x | 89.42x | 173.77x | 206.61x | 226.02x | 5/5 |
| chess960_materialize | 1.27x | 1.79x | 1.76x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.94x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.93x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.14x | 19.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.73x | 5.28x | 2.10x | 1.12x | 1.95x | 2.46x | 10/10 |
| corpus_materialize | 1.15x | 1.68x | 1.58x | 1.38x | 2.06x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.99x | 2.76x | 1.62x | 2.25x | 1.24x | 10/10 |
| replay_trusted_native | 0.93x | 0.58x | 1.49x | 1.47x | 1.63x | 1.77x | 8/10 |
| replay_checked_native | 0.98x | 2.43x | 1.06x | 8.23x | 7.22x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
