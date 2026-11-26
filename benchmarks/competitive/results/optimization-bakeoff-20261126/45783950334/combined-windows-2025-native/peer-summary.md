| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.24x | 1.47x | 1.44x | 2.10x | 1.80x | 30/30 |
| materialize | 1.00x | 1.91x | 1.31x | 1.19x | 2.21x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.91x | 1.54x | 1.37x | 2.30x | 1.96x | 30/30 |
| perft_full_d3 | 1.02x | 1.35x | 1.41x | 1.28x | 1.85x | 1.77x | 30/30 |
| fen_parse | 0.98x | 1.17x | 1.29x | 1.24x | 2.02x | 1.44x | 30/30 |
| fen_write_native_ep | 0.99x | 4.01x | 8.35x | 1.69x | 31.36x | 2.01x | 30/30 |
| snapshot | 1.30x | 1.61x | 1.18x | 4.04x | 1.18x | 1.06x | 29/30 |
| san_parse_all | 0.98x | 1.95x | N/A | 5.27x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.25x | N/A | 3.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.95x | 1.20x | 8.02x | 1.27x | 1.41x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.49x | 0.55x | 34.12x | 67.65x | 98.81x | 78.15x | 4/5 |
| chess960_materialize | 1.00x | 1.46x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.53x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 27.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 21.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.36x | 1.77x | 1.36x | 2.44x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.64x | 1.33x | 1.18x | 2.10x | 1.43x | 10/10 |
| corpus_uci_sorted | 1.14x | 1.80x | 2.21x | 1.82x | 2.17x | 1.42x | 10/10 |
| replay_trusted_native | 0.92x | 0.80x | 1.19x | 1.52x | 2.20x | 2.49x | 9/10 |
| replay_checked_native | 0.96x | 2.11x | 0.93x | 6.95x | 7.17x | 8.51x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
