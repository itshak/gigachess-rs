| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.87x | 1.71x | 1.71x | 2.41x | 2.12x | 30/30 |
| materialize | 1.01x | 2.03x | 1.38x | 1.29x | 2.36x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.13x | 1.62x | 1.40x | 2.32x | 2.08x | 30/30 |
| perft_full_d3 | 1.00x | 1.42x | 1.51x | 1.33x | 1.97x | 1.89x | 30/30 |
| fen_parse | 1.01x | 1.36x | 1.44x | 1.34x | 2.22x | 1.47x | 30/30 |
| fen_write_native_ep | 1.05x | 4.23x | 8.40x | 1.71x | 32.17x | 2.04x | 30/30 |
| snapshot | 0.99x | 1.92x | 1.05x | 4.07x | 1.02x | 1.01x | 23/30 |
| san_parse_all | 1.00x | 2.13x | N/A | 5.54x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.08x | N/A | 3.87x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.40x | 1.41x | 9.91x | 1.49x | 1.68x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.24x | 78.98x | 158.03x | 232.28x | 174.07x | 5/5 |
| chess960_materialize | 1.03x | 1.61x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 26.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 21.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.57x | 1.82x | 1.38x | 2.47x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.57x | 1.26x | 1.16x | 1.97x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.80x | 2.37x | 1.99x | 2.34x | 1.47x | 10/10 |
| replay_trusted_native | 1.02x | 0.86x | 1.64x | 2.44x | 2.95x | 3.40x | 9/10 |
| replay_checked_native | 0.87x | 2.40x | 0.96x | 7.46x | 7.14x | 8.52x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
