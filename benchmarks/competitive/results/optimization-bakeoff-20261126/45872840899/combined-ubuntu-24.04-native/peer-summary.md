| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.70x | 1.67x | 1.63x | 2.57x | 2.03x | 30/30 |
| materialize | 1.00x | 1.95x | 1.67x | 1.25x | 2.25x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.06x | 1.60x | 1.40x | 2.40x | 2.04x | 30/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.54x | 1.32x | 1.95x | 1.94x | 30/30 |
| fen_parse | 1.12x | 1.42x | 1.40x | 1.23x | 2.18x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.41x | 1.73x | 28.54x | 2.20x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 3.30x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.06x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.40x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.30x | 1.31x | 4.57x | 1.46x | 1.60x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.04x | 141.01x | 222.23x | 154.99x | 5/5 |
| chess960_materialize | 1.00x | 1.60x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.82x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 30.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 21.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.47x | 1.77x | 1.39x | 2.64x | 2.38x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.52x | 1.15x | 2.09x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.40x | 3.19x | 2.41x | 3.03x | 1.77x | 10/10 |
| replay_trusted_native | 0.96x | 0.87x | 1.58x | 1.88x | 2.52x | 2.88x | 9/10 |
| replay_checked_native | 0.96x | 2.36x | 1.06x | 7.91x | 7.91x | 9.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
