| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.02x | 1.98x | 1.55x | 1.37x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.00x | 1.63x | 1.47x | 2.09x | 2.13x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.81x | 1.51x | 2.21x | 2.10x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.27x | 1.74x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.30x | 1.57x | 29.71x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.09x | 1.02x | 1.47x | 1.01x | 0.95x | 19/30 |
| san_parse_all | 0.98x | 1.80x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.43x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.02x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.68x | 127.70x | 172.63x | 140.08x | 5/5 |
| chess960_materialize | 1.01x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.00x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.91x | 1.85x | 1.48x | 2.17x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.54x | 1.24x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.12x | 2.92x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.63x | 1.64x | 1.99x | 2.22x | 2.28x | 8/10 |
| replay_checked_native | 1.00x | 2.34x | 1.13x | 7.59x | 6.75x | 8.82x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
