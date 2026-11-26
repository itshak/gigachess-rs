| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.53x | 1.56x | 1.60x | 2.52x | 1.81x | 30/30 |
| materialize | 0.99x | 2.11x | 1.70x | 1.15x | 2.25x | 1.43x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.97x | 1.53x | 1.27x | 2.55x | 2.00x | 30/30 |
| perft_full_d3 | 0.98x | 1.24x | 1.29x | 0.97x | 1.98x | 1.48x | 26/30 |
| fen_parse | 1.12x | 1.51x | 1.64x | 1.33x | 2.03x | 1.24x | 30/30 |
| fen_write_native_ep | 1.00x | 4.30x | 9.87x | 1.86x | 27.91x | 2.35x | 30/30 |
| snapshot | 3.07x | 1.75x | 1.21x | 2.90x | 1.21x | 1.11x | 30/30 |
| san_parse_all | 0.98x | 2.25x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.39x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.59x | 1.03x | 3.59x | 1.17x | 1.34x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.15x | 84.46x | 147.02x | 191.28x | 124.97x | 5/5 |
| chess960_materialize | 1.03x | 1.82x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.87x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 17.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 4.24x | 1.64x | 1.26x | 2.29x | 1.86x | 9/10 |
| corpus_materialize | 0.98x | 1.89x | 1.71x | 1.24x | 2.81x | 1.51x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 3.17x | 2.34x | 2.91x | 1.65x | 10/10 |
| replay_trusted_native | 1.03x | 0.78x | 1.28x | 1.52x | 2.60x | 3.13x | 8/10 |
| replay_checked_native | 1.02x | 2.47x | 0.96x | 5.42x | 6.10x | 7.26x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
