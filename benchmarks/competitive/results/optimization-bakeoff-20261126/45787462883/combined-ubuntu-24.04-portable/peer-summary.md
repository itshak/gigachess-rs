| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.48x | 1.44x | 1.82x | 1.29x | 29/30 |
| materialize | 0.98x | 2.14x | 1.72x | 1.29x | 2.30x | 1.59x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.75x | 1.47x | 1.15x | 1.85x | 1.57x | 30/30 |
| perft_full_d3 | 0.97x | 1.25x | 1.48x | 1.15x | 1.77x | 1.65x | 30/30 |
| fen_parse | 0.91x | 1.37x | 1.38x | 1.11x | 1.74x | 1.20x | 30/30 |
| fen_write_native_ep | 0.99x | 2.66x | 9.37x | 1.87x | 26.16x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.21x | 2.81x | 1.14x | 0.92x | 24/30 |
| san_parse_all | 0.94x | 2.08x | N/A | 3.74x | N/A | N/A | 12/12 |
| san_render_native | 0.87x | 10.17x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.83x | 1.13x | 4.46x | 1.05x | 1.07x | 25/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 73.52x | 104.96x | 158.13x | 110.60x | 5/5 |
| chess960_materialize | 1.04x | 1.78x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.10x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 21.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 20.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.13x | 1.76x | 1.22x | 1.92x | 1.84x | 10/10 |
| corpus_materialize | 0.99x | 1.93x | 1.77x | 1.30x | 2.89x | 1.66x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.05x | 2.95x | 2.20x | 2.83x | 1.59x | 10/10 |
| replay_trusted_native | 1.07x | 0.74x | 1.47x | 1.61x | 2.37x | 2.15x | 8/10 |
| replay_checked_native | 0.95x | 2.19x | 0.95x | 5.17x | 5.59x | 6.85x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
