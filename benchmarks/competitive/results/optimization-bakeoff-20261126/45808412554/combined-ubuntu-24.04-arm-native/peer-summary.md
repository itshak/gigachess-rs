| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.29x | 1.63x | 1.75x | 2.12x | 2.13x | 30/30 |
| materialize | 1.05x | 1.91x | 1.50x | 1.31x | 2.10x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.05x | 1.64x | 1.48x | 2.08x | 2.14x | 30/30 |
| perft_full_d3 | 0.99x | 1.16x | 1.25x | 1.05x | 1.51x | 1.47x | 29/30 |
| fen_parse | 1.01x | 1.26x | 1.36x | 1.29x | 1.80x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.34x | 9.16x | 1.57x | 29.72x | 2.37x | 30/30 |
| snapshot | 0.93x | 1.13x | 1.07x | 1.51x | 1.08x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.74x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.17x | 1.39x | 3.80x | 1.13x | 1.13x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.17x | 68.95x | 117.05x | 154.56x | 124.15x | 5/5 |
| chess960_materialize | 1.07x | 1.61x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.86x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 5.13x | 1.97x | 1.56x | 2.29x | 2.52x | 10/10 |
| corpus_materialize | 1.02x | 1.61x | 1.51x | 1.21x | 1.90x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.91x | 2.10x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.53x | 1.37x | 1.63x | 1.82x | 1.88x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 0.98x | 6.60x | 5.88x | 7.60x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
