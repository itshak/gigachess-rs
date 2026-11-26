| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.20x | 1.54x | 1.50x | 2.37x | 1.87x | 30/30 |
| materialize | 1.01x | 1.84x | 1.23x | 1.06x | 2.19x | 1.38x | 29/30 |
| perft_bulk_d4 | 1.00x | 3.83x | 1.57x | 1.38x | 2.36x | 2.16x | 30/30 |
| perft_full_d3 | 1.00x | 1.28x | 1.31x | 1.09x | 1.75x | 1.61x | 27/30 |
| fen_parse | 1.20x | 1.56x | 1.63x | 1.48x | 2.44x | 1.67x | 30/30 |
| fen_write_native_ep | 0.95x | 3.84x | 7.59x | 1.58x | 30.91x | 2.04x | 30/30 |
| snapshot | 1.02x | 1.80x | 1.28x | 4.14x | 1.23x | 1.16x | 30/30 |
| san_parse_all | 0.94x | 1.90x | N/A | 4.50x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.51x | N/A | 3.38x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.53x | 1.04x | 6.96x | 1.22x | 1.35x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.20x | 1.20x | 101.98x | 159.79x | 273.75x | 179.02x | 5/5 |
| chess960_materialize | 1.00x | 1.47x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.65x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 15.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.63x | 1.87x | 1.32x | 2.47x | 2.16x | 10/10 |
| corpus_materialize | 1.00x | 1.78x | 1.36x | 1.18x | 2.87x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.89x | 2.43x | 2.05x | 2.31x | 1.51x | 10/10 |
| replay_trusted_native | 1.00x | 0.73x | 1.26x | 1.61x | 2.52x | 3.13x | 8/10 |
| replay_checked_native | 1.01x | 2.21x | 0.94x | 6.09x | 6.71x | 7.78x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
