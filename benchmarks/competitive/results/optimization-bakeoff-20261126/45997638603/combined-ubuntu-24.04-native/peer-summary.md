| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.48x | 1.90x | 1.63x | 2.29x | 1.98x | 30/30 |
| materialize | 1.03x | 2.01x | 1.63x | 1.59x | 2.32x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.00x | 1.62x | 1.24x | 2.26x | 1.95x | 29/30 |
| perft_full_d3 | 1.25x | 1.73x | 1.98x | 1.86x | 2.46x | 2.74x | 30/30 |
| fen_parse | 0.99x | 1.41x | 1.57x | 1.27x | 1.53x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 4.93x | 10.62x | 2.12x | 29.20x | 2.59x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.00x | 4.66x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 0.91x | 1.64x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.34x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.37x | 4.16x | 1.44x | 1.67x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 81.31x | 149.47x | 177.79x | 145.34x | 4/5 |
| chess960_materialize | 1.00x | 1.57x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.58x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 40.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 25.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.26x | 5.81x | 2.31x | 1.15x | 2.61x | 2.50x | 10/10 |
| corpus_materialize | 1.01x | 1.38x | 1.31x | 1.06x | 1.67x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 2.98x | 2.28x | 2.82x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 0.84x | 1.54x | 1.53x | 2.10x | 2.41x | 9/10 |
| replay_checked_native | 1.02x | 2.36x | 1.23x | 9.38x | 9.06x | 10.61x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
