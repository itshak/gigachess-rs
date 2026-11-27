| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.47x | 1.88x | 1.64x | 2.58x | 1.91x | 30/30 |
| materialize | 0.93x | 2.12x | 1.55x | 1.71x | 2.47x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.82x | 1.52x | 1.20x | 2.37x | 1.86x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.56x | 1.51x | 1.99x | 1.64x | 30/30 |
| fen_parse | 0.94x | 1.19x | 1.47x | 1.09x | 1.36x | 1.18x | 29/30 |
| fen_write_native_ep | 1.01x | 4.64x | 10.20x | 2.05x | 28.26x | 2.17x | 30/30 |
| snapshot | 1.00x | 2.00x | 0.99x | 4.65x | 1.00x | 1.33x | 21/30 |
| san_parse_all | 1.02x | 1.91x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.83x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.14x | 1.33x | 4.02x | 1.42x | 1.64x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 0.90x | 1.00x | 78.51x | 146.63x | 167.78x | 153.29x | 4/5 |
| chess960_materialize | 1.02x | 1.63x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.49x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 23.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.46x | 2.04x | 1.14x | 2.53x | 2.29x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.30x | 1.08x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 2.95x | 2.21x | 2.84x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.89x | 1.68x | 1.68x | 2.25x | 2.62x | 9/10 |
| replay_checked_native | 1.00x | 2.15x | 1.17x | 9.17x | 8.88x | 10.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
