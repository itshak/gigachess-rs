| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.18x | 4.40x | 1.41x | 1.49x | 1.89x | 1.67x | 30/30 |
| materialize | 1.10x | 1.92x | 1.46x | 1.08x | 2.39x | 1.28x | 28/30 |
| perft_bulk_d4 | 1.37x | 4.08x | 1.51x | 1.23x | 2.00x | 1.87x | 30/30 |
| perft_full_d3 | 2.40x | 1.51x | 1.62x | 1.26x | 1.89x | 1.89x | 30/30 |
| fen_parse | 1.07x | 1.23x | 1.29x | 1.28x | 1.57x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 3.08x | 6.65x | 1.28x | 20.48x | 1.59x | 30/30 |
| snapshot | 1.35x | 1.65x | 1.08x | 2.34x | 1.09x | 0.94x | 23/30 |
| san_parse_all | 1.86x | 1.92x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 1.77x | 11.62x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.34x | 0.82x | 0.86x | 4.02x | 0.83x | 0.92x | 7/30 |
| make_unmake_full | 1.32x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 1.36x | 1.20x | 67.74x | 132.53x | 153.61x | 148.19x | 5/5 |
| chess960_materialize | 1.14x | 1.52x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.41x | 3.15x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 37.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.16x | 20.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 6.16x | 2.08x | 1.34x | 2.50x | 2.64x | 10/10 |
| corpus_materialize | 1.03x | 1.58x | 1.35x | 1.13x | 1.82x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.51x | 1.26x | 1.80x | 1.29x | 1.68x | 0.99x | 9/10 |
| replay_trusted_native | 1.37x | 0.45x | 1.27x | 1.20x | 1.35x | 1.34x | 8/10 |
| replay_checked_native | 1.15x | 1.89x | 0.83x | 6.37x | 5.74x | 6.99x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
