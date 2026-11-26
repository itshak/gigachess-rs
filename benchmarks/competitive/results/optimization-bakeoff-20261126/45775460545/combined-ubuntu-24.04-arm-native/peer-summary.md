| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.67x | 1.40x | 1.50x | 1.82x | 1.81x | 30/30 |
| materialize | 1.00x | 1.77x | 1.38x | 1.21x | 1.95x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.58x | 1.46x | 1.33x | 1.87x | 1.91x | 30/30 |
| perft_full_d3 | 1.04x | 0.87x | 0.94x | 0.78x | 1.15x | 1.09x | 13/30 |
| fen_parse | 1.01x | 1.25x | 1.33x | 1.30x | 1.77x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 2.94x | 6.17x | 1.06x | 19.54x | 1.59x | 29/30 |
| snapshot | 1.00x | 1.14x | 1.08x | 1.51x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 1.06x | 1.90x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.50x | N/A | 2.48x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 0.89x | 1.06x | 2.89x | 0.87x | 0.86x | 11/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.30x | 76.60x | 124.11x | 172.46x | 141.16x | 5/5 |
| chess960_materialize | 1.05x | 1.48x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.53x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 24.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.59x | 1.77x | 1.43x | 2.07x | 2.28x | 10/10 |
| corpus_materialize | 1.00x | 1.58x | 1.51x | 1.20x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.95x | 1.39x | 1.86x | 0.97x | 8/10 |
| replay_trusted_native | 1.09x | 0.45x | 1.18x | 1.44x | 1.60x | 1.64x | 8/10 |
| replay_checked_native | 1.10x | 2.00x | 0.98x | 6.55x | 5.86x | 7.64x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
