| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.96x | 1.64x | 1.74x | 2.19x | 2.00x | 30/30 |
| materialize | 1.00x | 2.11x | 1.58x | 1.08x | 2.25x | 1.44x | 27/30 |
| perft_bulk_d4 | 0.96x | 4.18x | 1.49x | 1.22x | 1.87x | 1.81x | 30/30 |
| perft_full_d3 | 0.96x | 1.35x | 1.49x | 1.14x | 1.74x | 1.71x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.31x | 1.31x | 1.64x | 1.45x | 30/30 |
| fen_write_native_ep | 1.01x | 4.18x | 9.44x | 1.77x | 29.18x | 2.23x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.15x | 2.49x | 1.15x | 1.01x | 28/30 |
| san_parse_all | 0.94x | 1.75x | N/A | 3.53x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.67x | N/A | 2.61x | N/A | N/A | 12/12 |
| apply_children_full | 0.81x | 0.94x | 1.01x | 4.76x | 0.97x | 1.11x | 20/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 1.03x | 1.22x | 69.97x | 146.54x | 176.00x | 146.89x | 5/5 |
| chess960_materialize | 0.97x | 1.81x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 3.32x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 38.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.94x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.92x | 1.95x | 1.15x | 2.24x | 2.40x | 9/10 |
| corpus_materialize | 1.02x | 1.51x | 1.41x | 1.18x | 1.98x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.11x | 1.86x | 2.64x | 1.89x | 2.43x | 1.38x | 10/10 |
| replay_trusted_native | 1.05x | 0.63x | 1.60x | 1.69x | 1.80x | 1.75x | 8/10 |
| replay_checked_native | 1.02x | 2.47x | 1.12x | 8.69x | 7.75x | 9.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
