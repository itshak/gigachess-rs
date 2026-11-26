| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.12x | 1.47x | 1.45x | 2.16x | 1.78x | 30/30 |
| materialize | 1.00x | 1.80x | 1.54x | 1.16x | 2.09x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.75x | 1.49x | 1.31x | 2.25x | 1.87x | 30/30 |
| perft_full_d3 | 1.10x | 1.30x | 1.49x | 1.29x | 1.87x | 1.83x | 30/30 |
| fen_parse | 1.07x | 1.25x | 1.33x | 1.08x | 1.91x | 1.16x | 29/30 |
| fen_write_native_ep | 1.01x | 2.39x | 5.49x | 0.94x | 15.76x | 1.22x | 25/30 |
| snapshot | 1.76x | 1.98x | 1.25x | 3.64x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.03x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.63x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.92x | 1.11x | 3.87x | 1.23x | 1.35x | 28/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.16x | 1.24x | 77.13x | 146.45x | 227.14x | 154.89x | 5/5 |
| chess960_materialize | 1.01x | 1.43x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.46x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 28.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 19.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.41x | 1.81x | 1.36x | 2.61x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.58x | 1.20x | 2.04x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.44x | 3.23x | 2.46x | 3.16x | 1.74x | 10/10 |
| replay_trusted_native | 1.00x | 0.69x | 1.34x | 1.56x | 2.03x | 2.32x | 8/10 |
| replay_checked_native | 1.00x | 2.22x | 1.01x | 7.64x | 7.31x | 8.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
