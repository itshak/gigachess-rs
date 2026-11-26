| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.87x | 1.47x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.83x | 1.48x | 1.25x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.52x | 1.40x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.99x | 1.16x | 1.26x | 1.05x | 1.55x | 1.47x | 29/30 |
| fen_parse | 1.01x | 1.25x | 1.34x | 1.31x | 1.80x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.39x | 1.58x | 29.72x | 2.40x | 30/30 |
| snapshot | 0.97x | 1.15x | 1.12x | 1.59x | 1.12x | 1.03x | 29/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.89x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.17x | 1.40x | 3.84x | 1.12x | 1.12x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 0.87x | 1.16x | 67.33x | 115.84x | 147.72x | 119.60x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.77x | 1.82x | 1.47x | 2.14x | 2.35x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.49x | 1.21x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.88x | 2.07x | 2.82x | 1.45x | 10/10 |
| replay_trusted_native | 1.04x | 0.52x | 1.37x | 1.65x | 1.86x | 1.88x | 8/10 |
| replay_checked_native | 0.99x | 2.01x | 0.98x | 6.64x | 5.89x | 7.65x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
