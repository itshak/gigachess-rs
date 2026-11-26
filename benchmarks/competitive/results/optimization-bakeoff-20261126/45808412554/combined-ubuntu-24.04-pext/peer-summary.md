| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 4.68x | 1.97x | 1.70x | 2.67x | 2.01x | 30/30 |
| materialize | 1.10x | 2.30x | 1.71x | 1.81x | 2.73x | 2.00x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.28x | 1.69x | 1.34x | 2.66x | 2.09x | 30/30 |
| perft_full_d3 | 1.06x | 1.54x | 1.78x | 1.74x | 2.27x | 2.10x | 30/30 |
| fen_parse | 0.96x | 1.24x | 1.28x | 1.13x | 1.41x | 1.26x | 30/30 |
| fen_write_native_ep | 1.01x | 4.63x | 10.12x | 2.04x | 30.01x | 2.19x | 30/30 |
| snapshot | 0.50x | 1.50x | 1.16x | 2.00x | 1.16x | 1.00x | 25/30 |
| san_parse_all | 0.87x | 1.71x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.25x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.13x | 1.93x | 3.96x | 1.54x | 1.74x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.83x | 1.33x | 65.85x | 122.88x | 145.79x | 145.94x | 5/5 |
| chess960_materialize | 1.06x | 1.82x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.50x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 39.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 7.03x | 2.63x | 1.45x | 3.16x | 2.93x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.33x | 1.10x | 1.74x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.00x | 2.21x | 2.94x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.82x | 1.45x | 1.47x | 2.06x | 2.40x | 9/10 |
| replay_checked_native | 1.00x | 2.06x | 1.10x | 8.82x | 8.49x | 9.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
