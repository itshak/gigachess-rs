| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.53x | 1.44x | 2.18x | 1.89x | 30/30 |
| materialize | 1.03x | 2.05x | 1.33x | 1.15x | 2.40x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.76x | 1.40x | 1.25x | 2.22x | 2.02x | 30/30 |
| perft_full_d3 | 1.23x | 1.35x | 1.46x | 1.22x | 1.94x | 1.95x | 30/30 |
| fen_parse | 1.09x | 1.34x | 1.52x | 1.49x | 2.13x | 1.59x | 30/30 |
| fen_write_native_ep | 1.00x | 3.70x | 7.10x | 1.51x | 31.57x | 2.13x | 30/30 |
| snapshot | 1.44x | 2.51x | 1.50x | 5.45x | 1.50x | 1.51x | 30/30 |
| san_parse_all | 1.00x | 1.97x | N/A | 4.58x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 10.66x | N/A | 3.83x | N/A | N/A | 12/12 |
| apply_children_full | 1.21x | 1.61x | 1.36x | 7.73x | 1.54x | 1.94x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.25x | 88.86x | 173.06x | 241.50x | 186.52x | 5/5 |
| chess960_materialize | 1.17x | 1.62x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.69x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 33.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.38x | 23.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 5.75x | 1.63x | 1.10x | 2.24x | 2.11x | 9/10 |
| corpus_materialize | 0.98x | 1.60x | 1.22x | 1.06x | 2.20x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.64x | 1.93x | 1.73x | 1.91x | 1.38x | 10/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.21x | 1.40x | 2.14x | 2.58x | 8/10 |
| replay_checked_native | 1.01x | 2.02x | 0.91x | 7.06x | 7.50x | 8.81x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
