| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.06x | 1.68x | 1.63x | 2.34x | 1.91x | 30/30 |
| materialize | 0.97x | 2.01x | 1.64x | 1.05x | 2.17x | 1.33x | 28/30 |
| perft_bulk_d4 | 1.03x | 4.00x | 1.60x | 1.35x | 1.95x | 1.85x | 30/30 |
| perft_full_d3 | 1.02x | 1.54x | 1.76x | 1.27x | 1.91x | 1.87x | 30/30 |
| fen_parse | 1.07x | 1.25x | 1.36x | 1.26x | 1.82x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.37x | 13.58x | 1.68x | 28.91x | 2.14x | 30/30 |
| snapshot | 1.01x | 1.79x | 1.26x | 2.79x | 1.26x | 0.99x | 24/30 |
| san_parse_all | 0.96x | 1.68x | N/A | 3.59x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 12.99x | N/A | 2.86x | N/A | N/A | 12/12 |
| apply_children_full | 1.13x | 1.10x | 1.27x | 5.81x | 1.17x | 1.34x | 27/30 |
| make_unmake_full | 1.10x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.15x | 1.21x | 77.76x | 160.72x | 215.86x | 176.63x | 5/5 |
| chess960_materialize | 1.02x | 1.95x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 3.01x | 1.85x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 40.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.81x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.89x | 21.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 6.31x | 2.05x | 1.24x | 2.36x | 2.62x | 10/10 |
| corpus_materialize | 0.73x | 1.63x | 1.38x | 1.11x | 1.75x | 1.17x | 9/10 |
| corpus_uci_sorted | 0.77x | 1.81x | 2.46x | 1.81x | 2.31x | 1.48x | 10/10 |
| replay_trusted_native | 0.89x | 0.59x | 1.62x | 1.77x | 1.71x | 2.03x | 8/10 |
| replay_checked_native | 0.70x | 2.55x | 1.21x | 8.97x | 9.13x | 9.31x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
