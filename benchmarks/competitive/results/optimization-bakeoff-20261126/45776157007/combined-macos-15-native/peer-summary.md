| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.89x | 4.14x | 1.30x | 1.44x | 1.74x | 1.72x | 30/30 |
| materialize | 0.93x | 2.00x | 1.38x | 0.98x | 2.62x | 1.26x | 25/30 |
| perft_bulk_d4 | 0.91x | 3.86x | 1.41x | 1.23x | 1.83x | 1.80x | 27/30 |
| perft_full_d3 | 0.95x | 0.95x | 1.04x | 0.79x | 1.19x | 1.20x | 17/30 |
| fen_parse | 0.83x | 1.19x | 1.30x | 1.18x | 1.56x | 1.28x | 27/30 |
| fen_write_native_ep | 1.02x | 2.99x | 6.90x | 1.27x | 20.56x | 1.62x | 30/30 |
| snapshot | 0.88x | 1.31x | 0.93x | 1.84x | 0.90x | 0.87x | 19/30 |
| san_parse_all | 1.08x | 1.80x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 12.77x | N/A | 2.78x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 0.81x | 0.81x | 4.43x | 0.84x | 0.97x | 11/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.10x | N/A | N/A | 3/6 |
| startpos | 0.97x | 2.14x | 76.73x | 179.26x | 177.83x | 142.26x | 5/5 |
| chess960_materialize | 1.17x | 1.53x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.40x | 1.15x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 36.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.17x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 19.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 6.68x | 2.14x | 1.38x | 2.44x | 2.73x | 10/10 |
| corpus_materialize | 1.09x | 1.52x | 1.57x | 1.40x | 2.15x | 1.57x | 10/10 |
| corpus_uci_sorted | 1.27x | 1.72x | 2.25x | 1.64x | 2.20x | 1.16x | 10/10 |
| replay_trusted_native | 0.89x | 0.47x | 1.06x | 1.11x | 1.11x | 1.22x | 5/10 |
| replay_checked_native | 0.82x | 1.68x | 0.79x | 5.33x | 5.99x | 7.65x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
