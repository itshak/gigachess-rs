| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.40x | 1.73x | 2.03x | 2.60x | 2.08x | 30/30 |
| materialize | 0.89x | 1.99x | 1.53x | 1.08x | 2.23x | 1.35x | 28/30 |
| perft_bulk_d4 | 1.03x | 4.26x | 1.65x | 1.31x | 2.01x | 2.17x | 30/30 |
| perft_full_d3 | 1.10x | 1.76x | 2.00x | 1.53x | 2.31x | 2.36x | 30/30 |
| fen_parse | 1.01x | 1.22x | 1.31x | 1.32x | 1.77x | 1.62x | 29/30 |
| fen_write_native_ep | 0.95x | 4.38x | 9.12x | 1.77x | 27.35x | 2.38x | 30/30 |
| snapshot | 0.99x | 1.86x | 1.04x | 2.69x | 1.13x | 0.99x | 23/30 |
| san_parse_all | 0.92x | 1.67x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 12.00x | N/A | 2.87x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.16x | 1.18x | 5.92x | 1.15x | 1.25x | 27/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.37x | N/A | N/A | 5/6 |
| startpos | 0.53x | 0.82x | 58.54x | 102.73x | 131.81x | 147.71x | 4/5 |
| chess960_materialize | 0.88x | 1.70x | 1.80x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 3.24x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.81x | 39.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 1.17x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 21.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 5.66x | 2.02x | 1.40x | 2.26x | 2.62x | 9/10 |
| corpus_materialize | 0.84x | 1.51x | 1.32x | 1.06x | 1.95x | 1.24x | 9/10 |
| corpus_uci_sorted | 0.91x | 1.82x | 2.81x | 1.77x | 2.48x | 1.47x | 10/10 |
| replay_trusted_native | 1.05x | 0.58x | 1.65x | 1.57x | 1.68x | 1.75x | 8/10 |
| replay_checked_native | 0.94x | 2.70x | 1.21x | 10.12x | 8.00x | 10.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
