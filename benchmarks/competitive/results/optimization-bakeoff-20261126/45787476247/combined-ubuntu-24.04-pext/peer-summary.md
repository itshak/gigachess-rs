| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.71x | 1.64x | 1.64x | 2.54x | 1.88x | 30/30 |
| materialize | 1.00x | 2.14x | 1.68x | 1.16x | 2.20x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.18x | 1.61x | 1.34x | 2.58x | 2.10x | 30/30 |
| perft_full_d3 | 0.99x | 1.24x | 1.28x | 0.98x | 1.82x | 1.46x | 26/30 |
| fen_parse | 0.95x | 1.29x | 1.41x | 1.10x | 2.19x | 1.22x | 30/30 |
| fen_write_native_ep | 0.99x | 4.11x | 9.54x | 1.89x | 27.87x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.22x | 2.77x | 1.28x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 2.30x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 9.62x | N/A | 1.85x | N/A | N/A | 12/12 |
| apply_children_full | 0.85x | 1.33x | 0.89x | 3.34x | 0.98x | 1.50x | 21/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 98.37x | 133.40x | 230.76x | 145.27x | 5/5 |
| chess960_materialize | 0.99x | 1.79x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.04x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.01x | 1.90x | 1.47x | 2.66x | 2.18x | 10/10 |
| corpus_materialize | 1.01x | 2.09x | 1.78x | 1.28x | 2.91x | 1.55x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.33x | 3.16x | 2.35x | 3.00x | 1.66x | 10/10 |
| replay_trusted_native | 1.00x | 0.75x | 1.26x | 1.44x | 2.63x | 3.02x | 8/10 |
| replay_checked_native | 0.98x | 2.42x | 0.93x | 5.34x | 6.06x | 7.05x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
