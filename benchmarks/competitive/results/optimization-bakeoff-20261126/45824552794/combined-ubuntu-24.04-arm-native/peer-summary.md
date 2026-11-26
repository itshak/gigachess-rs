| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.89x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.81x | 1.43x | 1.25x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.39x | 1.27x | 1.78x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.28x | 1.07x | 1.57x | 1.48x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.28x | 1.75x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.45x | 1.63x | 30.27x | 2.40x | 30/30 |
| snapshot | 1.02x | 1.14x | 1.01x | 1.46x | 1.00x | 0.94x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.39x | 1.64x | 4.48x | 1.35x | 1.35x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.20x | 128.84x | 169.28x | 136.23x | 5/5 |
| chess960_materialize | 1.00x | 1.57x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.67x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.96x | 1.51x | 1.22x | 1.77x | 1.93x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.46x | 1.17x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.61x | 1.60x | 1.96x | 2.18x | 2.21x | 8/10 |
| replay_checked_native | 1.00x | 2.20x | 1.07x | 7.23x | 6.36x | 8.22x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
