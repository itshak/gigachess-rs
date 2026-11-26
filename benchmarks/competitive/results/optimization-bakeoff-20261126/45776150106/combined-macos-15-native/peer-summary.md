| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.25x | 1.36x | 1.38x | 1.77x | 1.58x | 30/30 |
| materialize | 1.04x | 2.04x | 1.47x | 1.12x | 2.33x | 1.31x | 28/30 |
| perft_bulk_d4 | 1.02x | 4.10x | 1.53x | 1.33x | 1.81x | 1.91x | 29/30 |
| perft_full_d3 | 0.95x | 0.87x | 0.93x | 0.72x | 0.99x | 1.03x | 8/30 |
| fen_parse | 0.98x | 1.21x | 1.38x | 1.25x | 1.66x | 1.46x | 29/30 |
| fen_write_native_ep | 0.96x | 2.99x | 6.24x | 1.32x | 19.58x | 1.70x | 30/30 |
| snapshot | 0.62x | 1.04x | 0.70x | 1.47x | 0.70x | 0.62x | 9/30 |
| san_parse_all | 0.98x | 1.71x | N/A | 3.65x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 11.67x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 0.83x | 0.74x | 0.77x | 3.79x | 0.85x | 0.82x | 8/30 |
| make_unmake_full | 0.76x | N/A | N/A | 0.86x | N/A | N/A | 1/6 |
| startpos | 0.66x | 0.75x | 47.19x | 130.14x | 133.68x | 107.56x | 4/5 |
| chess960_materialize | 0.89x | 1.53x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.72x | 2.54x | 1.20x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.71x | 28.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.90x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.72x | 15.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 6.33x | 2.13x | 1.43x | 2.54x | 2.54x | 10/10 |
| corpus_materialize | 1.11x | 1.54x | 1.47x | 1.17x | 1.96x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.15x | 1.31x | 2.01x | 1.44x | 1.65x | 1.04x | 9/10 |
| replay_trusted_native | 0.97x | 0.44x | 1.17x | 1.13x | 1.22x | 1.19x | 8/10 |
| replay_checked_native | 0.78x | 1.45x | 0.80x | 5.31x | 4.84x | 5.47x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
