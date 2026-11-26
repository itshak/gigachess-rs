| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.82x | 1.42x | 1.25x | 2.01x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.68x | 1.50x | 1.36x | 1.92x | 1.97x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.07x | 1.55x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.28x | 1.78x | 1.44x | 30/30 |
| fen_write_native_ep | 1.00x | 4.45x | 9.41x | 1.59x | 29.35x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.25x | 0.93x | 1.59x | 0.91x | 1.01x | 18/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.90x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.11x | 1.28x | 3.60x | 1.07x | 1.08x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 75.89x | 132.27x | 176.17x | 143.05x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.73x | 1.83x | 1.46x | 2.14x | 2.36x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.50x | 1.21x | 1.89x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.92x | 2.10x | 2.79x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.48x | 1.28x | 1.57x | 1.74x | 1.78x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.98x | 6.61x | 5.83x | 7.64x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
