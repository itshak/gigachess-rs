| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.80x | 1.88x | 1.88x | 2.45x | 1.94x | 30/30 |
| materialize | 1.03x | 2.17x | 1.71x | 1.12x | 2.21x | 1.43x | 28/30 |
| perft_bulk_d4 | 1.05x | 3.95x | 1.56x | 1.41x | 1.99x | 1.91x | 30/30 |
| perft_full_d3 | 1.02x | 1.52x | 1.73x | 1.36x | 1.86x | 2.09x | 30/30 |
| fen_parse | 1.12x | 1.28x | 1.54x | 1.35x | 1.99x | 1.57x | 30/30 |
| fen_write_native_ep | 1.07x | 4.30x | 13.70x | 1.70x | 32.77x | 2.23x | 30/30 |
| snapshot | 0.96x | 1.93x | 1.06x | 2.36x | 1.18x | 0.93x | 20/30 |
| san_parse_all | 1.03x | 1.95x | N/A | 4.62x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 12.53x | N/A | 3.01x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.15x | 1.17x | 5.96x | 1.21x | 1.33x | 28/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.19x | 0.93x | 78.16x | 159.42x | 194.43x | 165.31x | 4/5 |
| chess960_materialize | 1.21x | 1.90x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.32x | 3.41x | 1.84x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 42.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.92x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 22.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.75x | 4.46x | 1.54x | 0.76x | 1.57x | 1.51x | 8/10 |
| corpus_materialize | 1.09x | 1.62x | 1.82x | 1.32x | 2.07x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.93x | 2.52x | 1.71x | 2.23x | 1.48x | 10/10 |
| replay_trusted_native | 1.10x | 0.67x | 2.03x | 1.81x | 1.90x | 1.75x | 8/10 |
| replay_checked_native | 0.89x | 2.27x | 1.08x | 7.68x | 7.35x | 8.01x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
