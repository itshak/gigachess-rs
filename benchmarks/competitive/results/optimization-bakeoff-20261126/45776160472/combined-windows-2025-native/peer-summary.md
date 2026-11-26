| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.10x | 1.42x | 1.40x | 2.00x | 1.82x | 30/30 |
| materialize | 1.00x | 1.94x | 1.34x | 1.23x | 2.21x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.52x | 1.29x | 2.13x | 1.96x | 30/30 |
| perft_full_d3 | 0.99x | 0.78x | 0.81x | 0.74x | 1.07x | 1.04x | 7/30 |
| fen_parse | 1.07x | 1.31x | 1.38x | 1.33x | 2.21x | 1.53x | 30/30 |
| fen_write_native_ep | 1.55x | 3.94x | 7.78x | 1.54x | 29.76x | 1.91x | 30/30 |
| snapshot | 0.57x | 1.15x | 1.14x | 3.07x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 1.03x | 1.86x | N/A | 5.18x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 10.69x | N/A | 3.66x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.33x | 0.83x | 5.20x | 0.88x | 1.02x | 16/30 |
| make_unmake_full | 1.04x | N/A | N/A | 0.99x | N/A | N/A | 3/6 |
| startpos | 0.99x | 1.13x | 67.34x | 133.69x | 196.36x | 152.64x | 5/5 |
| chess960_materialize | 1.16x | 1.43x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.43x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 29.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 19.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.40x | 1.79x | 1.37x | 2.48x | 2.47x | 10/10 |
| corpus_materialize | 1.07x | 1.60x | 1.29x | 1.14x | 2.02x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.22x | 1.49x | 1.21x | 1.44x | 0.96x | 8/10 |
| replay_trusted_native | 1.14x | 0.51x | 0.99x | 1.28x | 1.62x | 1.88x | 7/10 |
| replay_checked_native | 1.13x | 2.00x | 0.89x | 7.04x | 6.87x | 8.28x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
