| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.68x | 1.42x | 2.18x | 1.74x | 30/30 |
| materialize | 1.00x | 2.06x | 1.55x | 1.64x | 2.39x | 1.77x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.54x | 1.17x | 2.29x | 1.86x | 30/30 |
| perft_full_d3 | 1.63x | 1.33x | 1.58x | 1.50x | 1.91x | 1.68x | 30/30 |
| fen_parse | 0.97x | 1.29x | 1.25x | 1.05x | 1.35x | 1.05x | 30/30 |
| fen_write_native_ep | 1.06x | 3.24x | 6.44x | 1.23x | 18.55x | 1.67x | 30/30 |
| snapshot | 0.71x | 1.58x | 1.25x | 2.83x | 1.24x | 1.15x | 30/30 |
| san_parse_all | 1.03x | 1.75x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.96x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.49x | 0.96x | 2.79x | 0.94x | 1.20x | 22/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.92x | N/A | N/A | 1/6 |
| startpos | 1.02x | 1.29x | 80.99x | 152.35x | 180.53x | 148.90x | 5/5 |
| chess960_materialize | 0.96x | 1.54x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.21x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.92x | 20.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.32x | 2.37x | 1.31x | 2.87x | 2.67x | 10/10 |
| corpus_materialize | 1.01x | 1.45x | 1.32x | 1.11x | 1.74x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.43x | 1.85x | 1.39x | 1.81x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.14x | 1.09x | 1.53x | 1.97x | 7/10 |
| replay_checked_native | 1.03x | 1.89x | 0.96x | 7.65x | 7.31x | 8.90x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
