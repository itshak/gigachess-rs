| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.53x | 1.89x | 1.65x | 2.48x | 1.93x | 30/30 |
| materialize | 1.05x | 2.29x | 1.74x | 1.76x | 2.84x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.07x | 1.63x | 1.20x | 2.33x | 1.88x | 29/30 |
| perft_full_d3 | 1.15x | 2.51x | 2.13x | 1.74x | 2.25x | 2.19x | 30/30 |
| fen_parse | 0.94x | 1.30x | 1.30x | 0.99x | 1.50x | 1.17x | 25/30 |
| fen_write_native_ep | 1.01x | 4.93x | 10.71x | 2.16x | 29.98x | 2.66x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.04x | 1.95x | N/A | 4.51x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.25x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.13x | 1.35x | 4.07x | 1.27x | 1.73x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 98.81x | 171.67x | 231.19x | 190.30x | 5/5 |
| chess960_materialize | 1.02x | 1.82x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.81x | 19.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.20x | 1.91x | 1.08x | 2.34x | 2.15x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.31x | 1.08x | 1.77x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.34x | 3.15x | 2.32x | 3.10x | 1.57x | 10/10 |
| replay_trusted_native | 0.89x | 0.81x | 1.39x | 1.42x | 2.02x | 2.32x | 8/10 |
| replay_checked_native | 0.84x | 1.84x | 0.94x | 7.63x | 7.51x | 8.73x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
