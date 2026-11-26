| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.30x | 1.56x | 1.51x | 2.35x | 1.89x | 30/30 |
| materialize | 1.00x | 1.94x | 1.29x | 1.08x | 2.19x | 1.39x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.97x | 1.52x | 1.35x | 2.31x | 2.09x | 30/30 |
| perft_full_d3 | 0.85x | 1.47x | 1.07x | 0.87x | 1.43x | 1.26x | 24/30 |
| fen_parse | 1.01x | 1.53x | 1.65x | 1.52x | 2.24x | 1.64x | 30/30 |
| fen_write_native_ep | 1.01x | 3.00x | 6.07x | 1.24x | 23.81x | 1.52x | 30/30 |
| snapshot | 1.00x | 1.64x | 1.25x | 2.71x | 1.22x | 1.11x | 30/30 |
| san_parse_all | 1.02x | 1.88x | N/A | 4.67x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 9.75x | N/A | 3.47x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.51x | 1.05x | 6.18x | 1.16x | 1.33x | 28/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.08x | 83.01x | 134.98x | 195.80x | 150.35x | 5/5 |
| chess960_materialize | 0.99x | 1.48x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.57x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 16.65x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.44x | 1.82x | 1.32x | 2.40x | 2.17x | 10/10 |
| corpus_materialize | 0.99x | 1.75x | 1.36x | 1.16x | 2.76x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.87x | 2.40x | 1.96x | 2.28x | 1.51x | 10/10 |
| replay_trusted_native | 1.01x | 0.77x | 1.35x | 1.61x | 2.70x | 3.32x | 8/10 |
| replay_checked_native | 0.98x | 2.10x | 0.94x | 5.20x | 6.06x | 7.58x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
