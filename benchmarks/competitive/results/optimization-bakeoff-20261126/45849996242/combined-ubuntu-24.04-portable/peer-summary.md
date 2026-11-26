| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.03x | 1.65x | 1.43x | 1.89x | 1.37x | 30/30 |
| materialize | 0.91x | 2.15x | 1.82x | 1.79x | 2.61x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.99x | 1.60x | 1.19x | 1.96x | 1.77x | 29/30 |
| perft_full_d3 | 1.10x | 1.51x | 1.78x | 1.57x | 2.04x | 1.79x | 30/30 |
| fen_parse | 1.00x | 1.74x | 1.47x | 1.18x | 1.65x | 1.20x | 30/30 |
| fen_write_native_ep | 0.98x | 2.97x | 9.78x | 1.93x | 26.87x | 2.29x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.01x | 1.90x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.40x | N/A | 2.24x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.38x | 4.08x | 1.32x | 1.51x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 58.46x | 109.38x | 127.53x | 106.17x | 5/5 |
| chess960_materialize | 1.00x | 1.81x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.14x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 38.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.07x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.21x | 2.13x | 1.17x | 2.26x | 2.89x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.41x | 1.13x | 1.85x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.99x | 2.95x | 2.20x | 2.91x | 1.58x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.60x | 1.58x | 2.20x | 2.03x | 9/10 |
| replay_checked_native | 1.01x | 1.90x | 1.07x | 8.22x | 8.04x | 9.23x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
