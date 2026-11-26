| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.73x | 1.73x | 1.66x | 2.61x | 2.04x | 30/30 |
| materialize | 1.01x | 2.16x | 1.77x | 1.17x | 2.22x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.17x | 1.64x | 1.36x | 2.60x | 2.22x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.56x | 1.22x | 2.28x | 1.88x | 30/30 |
| fen_parse | 1.00x | 1.37x | 1.52x | 1.17x | 2.00x | 1.25x | 30/30 |
| fen_write_native_ep | 0.99x | 4.01x | 10.31x | 1.85x | 28.93x | 2.14x | 30/30 |
| snapshot | 0.98x | 1.87x | 1.22x | 3.81x | 1.27x | 1.22x | 30/30 |
| san_parse_all | 1.00x | 2.29x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.64x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.01x | 1.30x | 5.46x | 1.52x | 1.74x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.21x | 97.10x | 137.75x | 212.51x | 140.30x | 5/5 |
| chess960_materialize | 1.02x | 1.92x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.24x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.38x | 1.78x | 1.31x | 2.38x | 2.06x | 9/10 |
| corpus_materialize | 0.98x | 1.85x | 1.79x | 1.21x | 2.83x | 1.52x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.34x | 3.13x | 2.31x | 2.91x | 1.65x | 10/10 |
| replay_trusted_native | 1.07x | 1.14x | 2.03x | 2.21x | 3.80x | 4.60x | 10/10 |
| replay_checked_native | 1.01x | 2.95x | 1.15x | 6.49x | 7.26x | 8.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
