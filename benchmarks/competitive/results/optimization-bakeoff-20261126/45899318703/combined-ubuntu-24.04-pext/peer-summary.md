| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.45x | 1.87x | 1.62x | 2.55x | 1.90x | 30/30 |
| materialize | 1.00x | 2.27x | 1.75x | 1.79x | 2.78x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.82x | 1.52x | 1.20x | 2.39x | 1.86x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.60x | 1.52x | 2.02x | 1.66x | 30/30 |
| fen_parse | 1.02x | 1.41x | 1.39x | 1.18x | 1.53x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 4.93x | 10.40x | 1.89x | 27.63x | 2.50x | 30/30 |
| snapshot | 1.01x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 1.01x | 1.88x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.91x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.35x | 4.04x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.89x | 0.99x | 78.26x | 149.71x | 176.20x | 142.91x | 4/5 |
| chess960_materialize | 0.94x | 1.68x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 23.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.41x | 2.02x | 1.11x | 2.49x | 2.23x | 10/10 |
| corpus_materialize | 1.02x | 1.43x | 1.33x | 1.09x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 2.97x | 2.21x | 2.93x | 1.57x | 10/10 |
| replay_trusted_native | 0.98x | 0.88x | 1.60x | 1.57x | 2.19x | 2.50x | 9/10 |
| replay_checked_native | 1.00x | 2.21x | 1.20x | 9.27x | 8.93x | 10.37x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
