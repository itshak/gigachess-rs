| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.28x | 1.84x | 1.56x | 2.39x | 1.89x | 30/30 |
| materialize | 1.00x | 2.21x | 1.67x | 1.75x | 2.60x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.55x | 1.17x | 2.31x | 1.85x | 29/30 |
| perft_full_d3 | 1.00x | 1.39x | 1.56x | 1.51x | 1.96x | 1.66x | 30/30 |
| fen_parse | 0.93x | 1.18x | 1.46x | 1.06x | 1.35x | 1.15x | 29/30 |
| fen_write_native_ep | 1.00x | 4.61x | 10.27x | 1.90x | 27.74x | 2.19x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.12x | 1.97x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.04x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.33x | 3.92x | 1.29x | 1.64x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.13x | 88.77x | 163.13x | 187.78x | 169.18x | 5/5 |
| chess960_materialize | 1.09x | 1.75x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.49x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.83x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.39x | 2.03x | 1.11x | 2.47x | 2.25x | 9/10 |
| corpus_materialize | 1.01x | 1.41x | 1.30x | 1.07x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.26x | 2.98x | 2.21x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 1.05x | 0.91x | 1.68x | 1.65x | 2.28x | 2.71x | 9/10 |
| replay_checked_native | 1.01x | 2.01x | 1.12x | 8.58x | 8.18x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
