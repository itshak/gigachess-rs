| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 4.17x | 1.75x | 1.52x | 2.37x | 1.79x | 30/30 |
| materialize | 1.00x | 2.01x | 1.57x | 1.68x | 2.49x | 1.97x | 30/30 |
| perft_bulk_d4 | 1.41x | 3.98x | 1.58x | 1.24x | 2.48x | 1.94x | 30/30 |
| perft_full_d3 | 2.55x | 1.50x | 1.80x | 1.67x | 2.16x | 1.77x | 30/30 |
| fen_parse | 0.96x | 1.27x | 1.29x | 1.01x | 1.43x | 1.17x | 28/30 |
| fen_write_native_ep | 1.00x | 3.08x | 6.68x | 1.34x | 19.06x | 1.69x | 30/30 |
| snapshot | 2.51x | 1.66x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.93x | 1.95x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 2.09x | 9.56x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.35x | 1.31x | 0.83x | 2.53x | 0.79x | 1.45x | 18/30 |
| make_unmake_full | 1.33x | N/A | N/A | 0.98x | N/A | N/A | 4/6 |
| startpos | 1.98x | 1.25x | 102.02x | 172.09x | 219.97x | 192.97x | 5/5 |
| chess960_materialize | 1.19x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.45x | 2.27x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 39.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.89x | 0.87x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.18x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.20x | 6.68x | 2.44x | 1.37x | 2.99x | 2.75x | 10/10 |
| corpus_materialize | 1.03x | 1.48x | 1.35x | 1.13x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.45x | 1.37x | 1.79x | 1.32x | 1.72x | 0.93x | 8/10 |
| replay_trusted_native | 1.31x | 0.64x | 1.18x | 1.13x | 1.61x | 1.86x | 7/10 |
| replay_checked_native | 1.35x | 1.82x | 1.00x | 7.73x | 7.47x | 8.76x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
