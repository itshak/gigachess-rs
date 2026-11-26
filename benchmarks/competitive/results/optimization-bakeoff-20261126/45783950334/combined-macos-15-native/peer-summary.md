| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 5.12x | 1.75x | 1.69x | 2.17x | 1.90x | 30/30 |
| materialize | 0.87x | 1.99x | 1.44x | 1.07x | 2.27x | 1.40x | 27/30 |
| perft_bulk_d4 | 1.00x | 4.79x | 1.59x | 1.31x | 2.11x | 2.19x | 30/30 |
| perft_full_d3 | 0.87x | 1.51x | 1.66x | 1.20x | 1.88x | 1.87x | 30/30 |
| fen_parse | 0.86x | 1.19x | 1.25x | 1.20x | 1.57x | 1.35x | 27/30 |
| fen_write_native_ep | 0.90x | 4.01x | 8.61x | 1.68x | 29.20x | 2.19x | 30/30 |
| snapshot | 0.94x | 1.75x | 1.04x | 2.26x | 1.14x | 0.89x | 21/30 |
| san_parse_all | 1.03x | 2.02x | N/A | 4.59x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 12.37x | N/A | 2.81x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.01x | 1.02x | 4.82x | 1.03x | 1.12x | 21/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 5/6 |
| startpos | 0.90x | 0.91x | 72.66x | 147.68x | 175.31x | 140.52x | 4/5 |
| chess960_materialize | 0.88x | 1.74x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 3.36x | 1.74x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.82x | 17.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.79x | 6.49x | 1.97x | 1.19x | 2.13x | 2.35x | 10/10 |
| corpus_materialize | 0.89x | 1.44x | 1.34x | 1.18x | 1.92x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.75x | 1.83x | 2.43x | 1.83x | 2.50x | 1.41x | 10/10 |
| replay_trusted_native | 1.00x | 0.57x | 1.44x | 1.53x | 1.66x | 1.68x | 8/10 |
| replay_checked_native | 0.96x | 2.30x | 1.25x | 9.98x | 8.63x | 11.10x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
