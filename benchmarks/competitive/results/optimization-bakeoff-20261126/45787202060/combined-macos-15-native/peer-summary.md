| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.88x | 4.05x | 1.31x | 1.53x | 1.87x | 1.65x | 29/30 |
| materialize | 1.01x | 1.96x | 1.52x | 1.03x | 2.22x | 1.23x | 27/30 |
| perft_bulk_d4 | 0.94x | 3.34x | 1.20x | 0.96x | 1.50x | 1.50x | 25/30 |
| perft_full_d3 | 0.96x | 1.50x | 1.41x | 1.17x | 1.70x | 1.72x | 29/30 |
| fen_parse | 0.92x | 1.12x | 1.27x | 1.26x | 1.44x | 1.35x | 28/30 |
| fen_write_native_ep | 1.11x | 4.31x | 8.80x | 1.73x | 30.46x | 2.19x | 30/30 |
| snapshot | 1.42x | 1.95x | 1.28x | 2.63x | 1.39x | 1.15x | 27/30 |
| san_parse_all | 1.05x | 1.82x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 1.13x | 11.68x | N/A | 2.61x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 0.85x | 0.91x | 4.48x | 0.85x | 0.97x | 8/30 |
| make_unmake_full | 0.60x | N/A | N/A | 0.64x | N/A | N/A | 0/6 |
| startpos | 1.29x | 1.31x | 94.22x | 174.01x | 198.57x | 160.00x | 5/5 |
| chess960_materialize | 1.20x | 1.31x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.82x | 2.27x | 1.10x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 41.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.12x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 25.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.77x | 5.43x | 2.21x | 1.42x | 2.26x | 2.47x | 10/10 |
| corpus_materialize | 0.88x | 1.41x | 1.23x | 1.00x | 1.80x | 1.13x | 7/10 |
| corpus_uci_sorted | 0.78x | 1.74x | 1.97x | 1.89x | 2.31x | 1.26x | 10/10 |
| replay_trusted_native | 0.91x | 0.55x | 1.63x | 1.49x | 1.53x | 1.48x | 8/10 |
| replay_checked_native | 0.88x | 1.82x | 0.88x | 6.02x | 5.68x | 7.04x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
