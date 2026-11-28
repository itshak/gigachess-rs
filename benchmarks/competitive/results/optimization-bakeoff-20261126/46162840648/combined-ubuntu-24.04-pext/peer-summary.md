| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.74x | 1.97x | 1.72x | 2.78x | 2.00x | 30/30 |
| materialize | 1.02x | 2.31x | 1.80x | 1.78x | 3.00x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.01x | 1.59x | 1.25x | 2.60x | 1.95x | 29/30 |
| perft_full_d3 | 1.18x | 1.60x | 1.84x | 1.77x | 2.33x | 1.97x | 30/30 |
| fen_parse | 1.01x | 1.44x | 1.59x | 1.18x | 1.47x | 1.17x | 30/30 |
| fen_write_native_ep | 1.00x | 5.24x | 10.52x | 2.15x | 29.32x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.11x | 1.88x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.09x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.15x | 1.36x | 4.09x | 1.35x | 1.69x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.11x | 87.90x | 165.38x | 186.36x | 156.80x | 5/5 |
| chess960_materialize | 0.95x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.59x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.68x | 1.61x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.22x | 23.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 6.02x | 2.22x | 1.23x | 2.74x | 2.47x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.32x | 1.10x | 1.77x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 2.98x | 2.23x | 2.92x | 1.57x | 10/10 |
| replay_trusted_native | 0.90x | 0.79x | 1.41x | 1.40x | 1.95x | 2.22x | 8/10 |
| replay_checked_native | 0.92x | 2.05x | 1.07x | 8.50x | 8.27x | 9.48x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
