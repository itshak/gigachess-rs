| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.28x | 1.40x | 1.50x | 1.88x | 1.73x | 30/30 |
| materialize | 1.06x | 1.86x | 1.47x | 1.01x | 2.15x | 1.20x | 27/30 |
| perft_bulk_d4 | 1.17x | 4.29x | 1.58x | 1.30x | 2.03x | 2.05x | 30/30 |
| perft_full_d3 | 1.13x | 1.52x | 1.67x | 1.32x | 1.92x | 1.93x | 29/30 |
| fen_parse | 1.32x | 1.60x | 1.85x | 1.62x | 2.23x | 1.89x | 30/30 |
| fen_write_native_ep | 1.02x | 4.28x | 9.04x | 1.73x | 27.31x | 2.19x | 30/30 |
| snapshot | 1.11x | 1.63x | 1.22x | 2.30x | 1.07x | 0.96x | 24/30 |
| san_parse_all | 1.02x | 1.77x | N/A | 3.63x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.08x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 0.99x | 1.04x | 4.79x | 0.98x | 1.11x | 20/30 |
| make_unmake_full | 1.24x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.60x | 69.97x | 138.56x | 179.52x | 144.48x | 5/5 |
| chess960_materialize | 1.10x | 1.69x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.14x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 37.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.18x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.23x | 19.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.24x | 2.09x | 1.33x | 2.48x | 2.51x | 10/10 |
| corpus_materialize | 1.02x | 1.71x | 1.42x | 1.07x | 1.77x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.65x | 2.33x | 1.61x | 2.12x | 1.44x | 10/10 |
| replay_trusted_native | 0.87x | 0.57x | 1.34x | 1.42x | 1.42x | 1.52x | 8/10 |
| replay_checked_native | 1.13x | 2.08x | 0.91x | 6.73x | 5.94x | 7.39x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
