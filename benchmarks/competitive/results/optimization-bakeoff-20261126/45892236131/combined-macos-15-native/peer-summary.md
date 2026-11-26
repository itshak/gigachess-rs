| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.78x | 4.89x | 1.61x | 1.60x | 2.29x | 2.03x | 29/30 |
| materialize | 1.00x | 2.18x | 1.61x | 1.13x | 2.26x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.96x | 3.59x | 1.30x | 1.16x | 1.85x | 1.73x | 28/30 |
| perft_full_d3 | 0.72x | 1.17x | 1.24x | 1.00x | 1.38x | 1.45x | 28/30 |
| fen_parse | 0.98x | 1.21x | 1.28x | 1.24x | 1.52x | 1.28x | 29/30 |
| fen_write_native_ep | 0.84x | 3.77x | 7.61x | 1.41x | 23.77x | 1.88x | 30/30 |
| snapshot | 1.25x | 1.92x | 1.23x | 2.87x | 1.57x | 1.18x | 26/30 |
| san_parse_all | 1.01x | 1.90x | N/A | 4.08x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 11.77x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.09x | 1.15x | 5.61x | 1.13x | 1.19x | 23/30 |
| make_unmake_full | 0.72x | N/A | N/A | 1.01x | N/A | N/A | 3/6 |
| startpos | 2.32x | 2.98x | 165.77x | 328.55x | 381.63x | 342.40x | 5/5 |
| chess960_materialize | 0.92x | 1.84x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.49x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 45.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 0.89x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.10x | 19.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 5.49x | 1.95x | 1.30x | 3.11x | 2.39x | 10/10 |
| corpus_materialize | 0.99x | 1.51x | 1.33x | 1.28x | 2.03x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.80x | 2.43x | 1.80x | 2.12x | 1.35x | 10/10 |
| replay_trusted_native | 0.94x | 0.64x | 1.65x | 1.66x | 1.64x | 1.90x | 8/10 |
| replay_checked_native | 0.85x | 2.32x | 1.01x | 8.12x | 6.72x | 9.32x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
