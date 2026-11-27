| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.89x | 1.80x | 1.75x | 2.79x | 2.20x | 30/30 |
| materialize | 1.01x | 2.03x | 1.35x | 1.16x | 2.33x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.15x | 1.68x | 1.48x | 2.55x | 2.31x | 30/30 |
| perft_full_d3 | 1.31x | 1.97x | 2.02x | 1.69x | 2.76x | 2.49x | 30/30 |
| fen_parse | 1.09x | 1.55x | 1.61x | 1.51x | 2.48x | 1.66x | 30/30 |
| fen_write_native_ep | 1.00x | 3.95x | 7.58x | 1.60x | 31.80x | 2.00x | 30/30 |
| snapshot | 1.02x | 1.80x | 1.25x | 4.18x | 1.25x | 1.20x | 30/30 |
| san_parse_all | 1.02x | 1.95x | N/A | 4.77x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.25x | N/A | 3.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.95x | 1.52x | 8.50x | 1.88x | 2.05x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 82.57x | 131.26x | 202.64x | 148.94x | 5/5 |
| chess960_materialize | 1.01x | 1.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.70x | 1.89x | 1.39x | 2.51x | 2.27x | 10/10 |
| corpus_materialize | 1.01x | 1.87x | 1.40x | 1.19x | 2.83x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.89x | 2.39x | 1.97x | 2.29x | 1.51x | 10/10 |
| replay_trusted_native | 0.94x | 1.03x | 1.78x | 2.20x | 3.54x | 4.37x | 9/10 |
| replay_checked_native | 1.02x | 2.58x | 1.11x | 6.19x | 7.55x | 8.86x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
