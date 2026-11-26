| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.47x | 1.58x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.82x | 1.42x | 1.25x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.17x | 1.28x | 1.19x | 1.65x | 1.68x | 30/30 |
| perft_full_d3 | 0.95x | 1.10x | 1.20x | 1.01x | 1.44x | 1.39x | 27/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.27x | 1.78x | 1.43x | 30/30 |
| fen_write_native_ep | 0.99x | 4.48x | 9.20x | 1.59x | 29.84x | 2.38x | 30/30 |
| snapshot | 1.00x | 1.18x | 0.95x | 1.50x | 0.92x | 0.96x | 12/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 10.01x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 0.97x | 1.14x | 3.15x | 0.94x | 0.94x | 16/30 |
| make_unmake_full | 0.73x | N/A | N/A | 1.05x | N/A | N/A | 3/6 |
| startpos | 0.97x | 1.15x | 71.15x | 121.54x | 162.16x | 131.40x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.33x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.81x | 1.86x | 1.45x | 2.15x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.53x | 1.20x | 1.89x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.93x | 2.09x | 2.82x | 1.46x | 10/10 |
| replay_trusted_native | 0.92x | 0.48x | 1.22x | 1.48x | 1.63x | 1.68x | 8/10 |
| replay_checked_native | 0.90x | 1.81x | 0.88x | 5.91x | 5.33x | 6.87x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
