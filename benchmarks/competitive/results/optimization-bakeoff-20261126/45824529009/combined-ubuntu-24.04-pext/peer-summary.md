| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.11x | 1.72x | 1.50x | 2.31x | 1.76x | 30/30 |
| materialize | 1.00x | 2.15x | 1.59x | 1.64x | 2.53x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.37x | 1.34x | 1.05x | 2.07x | 1.65x | 28/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.74x | 1.67x | 2.14x | 1.76x | 30/30 |
| fen_parse | 0.94x | 1.23x | 1.28x | 0.98x | 1.36x | 1.06x | 24/30 |
| fen_write_native_ep | 1.00x | 4.89x | 10.47x | 2.13x | 28.66x | 2.71x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.92x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.10x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.03x | 1.27x | 3.87x | 1.27x | 1.61x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.11x | 88.13x | 152.28x | 192.57x | 160.32x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.27x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.68x | 1.74x | 0.97x | 2.13x | 1.94x | 8/10 |
| corpus_materialize | 1.00x | 1.37x | 1.26x | 1.06x | 1.68x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.34x | 3.01x | 2.24x | 2.93x | 1.56x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.59x | 1.52x | 2.14x | 2.47x | 9/10 |
| replay_checked_native | 1.00x | 2.02x | 1.11x | 8.50x | 8.19x | 9.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
