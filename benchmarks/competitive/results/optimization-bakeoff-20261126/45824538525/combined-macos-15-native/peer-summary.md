| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.40x | 1.42x | 1.51x | 1.88x | 1.94x | 29/30 |
| materialize | 0.99x | 1.95x | 1.38x | 1.07x | 2.20x | 1.35x | 29/30 |
| perft_bulk_d4 | 0.90x | 3.56x | 1.33x | 1.11x | 1.66x | 1.69x | 28/30 |
| perft_full_d3 | 0.92x | 1.52x | 1.71x | 1.33x | 1.89x | 1.95x | 30/30 |
| fen_parse | 0.98x | 1.26x | 1.38x | 1.47x | 1.68x | 1.51x | 30/30 |
| fen_write_native_ep | 0.94x | 4.46x | 9.54x | 1.85x | 29.86x | 2.43x | 30/30 |
| snapshot | 1.03x | 1.71x | 1.17x | 2.45x | 1.13x | 1.05x | 26/30 |
| san_parse_all | 0.90x | 1.72x | N/A | 3.75x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.60x | N/A | 2.77x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.16x | 1.27x | 5.90x | 1.17x | 1.34x | 27/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.01x | 62.88x | 130.42x | 155.75x | 166.15x | 5/5 |
| chess960_materialize | 0.79x | 1.70x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 2.66x | 1.12x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 0.88x | 42.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 17.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.80x | 4.72x | 1.53x | 0.99x | 1.72x | 1.89x | 9/10 |
| corpus_materialize | 0.86x | 1.39x | 1.41x | 1.18x | 1.87x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.87x | 1.74x | 2.35x | 2.09x | 2.32x | 1.36x | 10/10 |
| replay_trusted_native | 0.84x | 0.65x | 1.63x | 1.74x | 1.83x | 1.90x | 8/10 |
| replay_checked_native | 0.73x | 2.50x | 1.10x | 8.33x | 7.77x | 9.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
