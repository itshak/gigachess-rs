| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.88x | 1.63x | 2.50x | 1.92x | 30/30 |
| materialize | 1.02x | 2.27x | 1.75x | 1.79x | 2.77x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.82x | 1.52x | 1.19x | 2.37x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.55x | 1.52x | 1.98x | 1.62x | 30/30 |
| fen_parse | 1.00x | 1.25x | 1.32x | 1.14x | 1.44x | 1.25x | 30/30 |
| fen_write_native_ep | 0.99x | 4.90x | 10.06x | 1.86x | 28.65x | 2.17x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.07x | 1.86x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.04x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.13x | 1.34x | 4.06x | 1.34x | 1.68x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.13x | 88.70x | 164.86x | 197.86x | 180.17x | 5/5 |
| chess960_materialize | 0.95x | 1.70x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 36.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.12x | 1.89x | 1.05x | 2.35x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.32x | 1.09x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.32x | 3.07x | 2.24x | 2.99x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.88x | 1.60x | 1.62x | 2.22x | 2.58x | 9/10 |
| replay_checked_native | 1.11x | 2.23x | 1.18x | 9.45x | 9.23x | 10.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
