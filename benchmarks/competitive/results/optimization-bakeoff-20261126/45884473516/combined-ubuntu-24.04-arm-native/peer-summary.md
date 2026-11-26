| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.68x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.01x | 1.94x | 1.54x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.83x | 1.58x | 1.42x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 0.99x | 1.46x | 1.58x | 1.31x | 1.93x | 1.83x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.32x | 1.26x | 1.72x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.34x | 1.57x | 30.03x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.11x | 1.05x | 1.47x | 1.03x | 0.95x | 23/30 |
| san_parse_all | 0.97x | 1.81x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.50x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.50x | 127.44x | 172.48x | 141.33x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.93x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.39x | 1.71x | 1.35x | 1.97x | 2.18x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.54x | 1.23x | 1.89x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 3.12x | 2.08x | 2.88x | 1.46x | 10/10 |
| replay_trusted_native | 0.94x | 0.60x | 1.54x | 1.88x | 2.09x | 2.13x | 8/10 |
| replay_checked_native | 0.98x | 2.26x | 1.10x | 7.42x | 6.63x | 8.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
