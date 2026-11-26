| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.19x | 1.54x | 1.50x | 2.45x | 1.87x | 30/30 |
| materialize | 1.00x | 1.90x | 1.26x | 1.08x | 2.23x | 1.42x | 28/30 |
| perft_bulk_d4 | 1.02x | 3.64x | 1.48x | 1.30x | 2.25x | 2.06x | 30/30 |
| perft_full_d3 | 0.96x | 1.26x | 1.29x | 1.05x | 1.77x | 1.56x | 27/30 |
| fen_parse | 1.16x | 1.48x | 1.58x | 1.49x | 2.33x | 1.59x | 30/30 |
| fen_write_native_ep | 0.95x | 3.83x | 7.26x | 1.52x | 29.67x | 1.91x | 30/30 |
| snapshot | 1.00x | 1.77x | 1.28x | 4.14x | 1.28x | 1.18x | 30/30 |
| san_parse_all | 1.01x | 1.98x | N/A | 4.80x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 8.88x | N/A | 3.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.95x | 1.30x | 8.29x | 1.54x | 1.71x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 84.74x | 131.56x | 203.42x | 144.62x | 5/5 |
| chess960_materialize | 1.00x | 1.53x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.68x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.22x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.03x | 1.63x | 1.21x | 2.16x | 1.95x | 9/10 |
| corpus_materialize | 1.10x | 1.75x | 1.31x | 1.12x | 2.68x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.88x | 2.41x | 1.99x | 2.30x | 1.51x | 10/10 |
| replay_trusted_native | 1.16x | 1.18x | 2.05x | 2.54x | 4.10x | 5.05x | 10/10 |
| replay_checked_native | 1.02x | 2.31x | 1.02x | 5.79x | 7.23x | 7.91x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
