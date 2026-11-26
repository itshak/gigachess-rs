| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.30x | 1.47x | 1.51x | 2.31x | 1.75x | 30/30 |
| materialize | 1.00x | 1.87x | 1.57x | 1.26x | 2.21x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.66x | 1.41x | 1.31x | 2.23x | 1.72x | 30/30 |
| perft_full_d3 | 0.98x | 1.23x | 1.39x | 1.22x | 1.80x | 1.67x | 30/30 |
| fen_parse | 1.11x | 1.36x | 1.42x | 1.11x | 2.19x | 1.31x | 30/30 |
| fen_write_native_ep | 0.99x | 4.19x | 9.79x | 1.74x | 26.68x | 2.10x | 30/30 |
| snapshot | 1.01x | 1.99x | 1.25x | 3.81x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.10x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.34x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.11x | 1.23x | 4.20x | 1.35x | 1.53x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.26x | 77.65x | 132.73x | 237.28x | 160.42x | 5/5 |
| chess960_materialize | 1.00x | 1.48x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.60x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 27.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.12x | 1.64x | 1.30x | 2.47x | 2.15x | 10/10 |
| corpus_materialize | 0.98x | 1.51x | 1.44x | 1.12x | 1.93x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.46x | 3.18x | 2.42x | 3.05x | 1.73x | 10/10 |
| replay_trusted_native | 1.00x | 0.82x | 1.47x | 1.80x | 2.39x | 2.60x | 9/10 |
| replay_checked_native | 0.99x | 2.31x | 1.04x | 8.01x | 7.80x | 8.99x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
