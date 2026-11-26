| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.65x | 1.48x | 1.29x | 1.73x | 1.23x | 28/30 |
| materialize | 1.07x | 2.23x | 1.80x | 1.79x | 2.59x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.89x | 1.54x | 1.15x | 1.91x | 1.71x | 30/30 |
| perft_full_d3 | 1.00x | 0.95x | 1.12x | 1.00x | 1.39x | 1.15x | 22/30 |
| fen_parse | 0.99x | 1.71x | 1.50x | 1.16x | 1.54x | 1.22x | 30/30 |
| fen_write_native_ep | 1.01x | 1.98x | 6.46x | 1.26x | 17.85x | 1.53x | 30/30 |
| snapshot | 1.12x | 3.10x | 2.51x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.68x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.05x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.64x | 1.06x | 3.11x | 1.37x | 1.17x | 28/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.01x | N/A | N/A | 3/6 |
| startpos | 1.00x | 1.36x | 57.21x | 106.49x | 130.20x | 108.01x | 5/5 |
| chess960_materialize | 1.00x | 1.57x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.81x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.06x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 22.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.51x | 2.27x | 1.23x | 2.30x | 3.09x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.41x | 1.14x | 1.85x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.22x | 1.83x | 1.37x | 1.81x | 0.97x | 8/10 |
| replay_trusted_native | 1.05x | 0.62x | 1.18x | 1.17x | 1.60x | 1.46x | 7/10 |
| replay_checked_native | 1.03x | 1.87x | 0.96x | 7.52x | 7.35x | 8.42x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
