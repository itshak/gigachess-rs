| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.47x | 1.87x | 1.62x | 2.46x | 1.89x | 30/30 |
| materialize | 1.02x | 2.28x | 1.69x | 1.77x | 2.77x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.52x | 1.19x | 2.33x | 1.86x | 29/30 |
| perft_full_d3 | 1.00x | 1.30x | 1.48x | 1.42x | 1.85x | 1.58x | 30/30 |
| fen_parse | 0.99x | 1.29x | 1.30x | 1.07x | 1.46x | 1.05x | 29/30 |
| fen_write_native_ep | 1.00x | 4.58x | 10.32x | 2.16x | 30.50x | 2.53x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.21x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.06x | 1.34x | 3.96x | 1.22x | 2.05x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.25x | 99.10x | 186.65x | 228.71x | 177.50x | 5/5 |
| chess960_materialize | 1.01x | 1.78x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 36.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.96x | 18.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.05x | 1.87x | 1.05x | 2.27x | 2.07x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.31x | 1.10x | 1.77x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.22x | 2.97x | 2.21x | 2.89x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.92x | 1.57x | 1.60x | 2.25x | 2.58x | 9/10 |
| replay_checked_native | 1.00x | 2.05x | 1.04x | 8.56x | 8.23x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
