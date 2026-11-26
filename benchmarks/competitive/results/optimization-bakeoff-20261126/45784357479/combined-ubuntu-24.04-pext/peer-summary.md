| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.30x | 1.79x | 1.56x | 2.37x | 1.83x | 30/30 |
| materialize | 0.95x | 2.09x | 1.55x | 1.74x | 2.47x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.04x | 1.60x | 1.26x | 2.48x | 1.97x | 30/30 |
| perft_full_d3 | 0.99x | 1.49x | 1.69x | 1.68x | 2.24x | 1.80x | 30/30 |
| fen_parse | 0.99x | 1.23x | 1.31x | 0.99x | 1.46x | 1.15x | 27/30 |
| fen_write_native_ep | 0.99x | 4.93x | 10.72x | 2.14x | 30.47x | 2.62x | 30/30 |
| snapshot | 1.09x | 1.54x | 1.00x | 3.47x | 0.99x | 1.15x | 20/30 |
| san_parse_all | 1.00x | 1.90x | N/A | 4.52x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.14x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.73x | 1.08x | 3.18x | 1.10x | 1.30x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.08x | 72.18x | 124.54x | 165.15x | 138.55x | 5/5 |
| chess960_materialize | 0.86x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.38x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 0.91x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.04x | 19.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.79x | 2.53x | 1.40x | 3.06x | 2.81x | 10/10 |
| corpus_materialize | 1.00x | 1.47x | 1.36x | 1.12x | 1.80x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 3.17x | 2.29x | 3.08x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.24x | 1.26x | 1.82x | 2.07x | 8/10 |
| replay_checked_native | 1.01x | 2.01x | 1.05x | 8.46x | 8.23x | 9.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
