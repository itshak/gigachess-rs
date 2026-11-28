| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.88x | 1.82x | 1.72x | 2.65x | 2.18x | 30/30 |
| materialize | 0.99x | 1.98x | 1.32x | 1.12x | 2.31x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.11x | 1.69x | 1.51x | 2.56x | 2.29x | 30/30 |
| perft_full_d3 | 1.29x | 1.97x | 2.03x | 1.75x | 2.75x | 2.53x | 30/30 |
| fen_parse | 0.96x | 1.35x | 1.41x | 1.38x | 2.19x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.01x | 7.47x | 1.56x | 31.28x | 1.97x | 30/30 |
| snapshot | 0.99x | 1.78x | 1.28x | 4.12x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 1.00x | 1.98x | N/A | 5.06x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.43x | N/A | 3.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.00x | 1.35x | 8.58x | 1.58x | 1.76x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 83.44x | 138.75x | 204.89x | 142.44x | 5/5 |
| chess960_materialize | 0.98x | 1.66x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.84x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.69x | 1.92x | 1.37x | 2.50x | 2.21x | 10/10 |
| corpus_materialize | 0.99x | 1.90x | 1.40x | 1.20x | 2.91x | 1.56x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.90x | 2.41x | 1.97x | 2.29x | 1.51x | 10/10 |
| replay_trusted_native | 0.93x | 1.01x | 1.79x | 2.15x | 3.45x | 4.20x | 9/10 |
| replay_checked_native | 0.96x | 2.47x | 1.05x | 6.17x | 7.65x | 8.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
