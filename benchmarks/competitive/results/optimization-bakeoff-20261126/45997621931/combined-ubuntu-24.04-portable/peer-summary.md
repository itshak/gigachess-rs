| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.65x | 1.44x | 1.94x | 1.38x | 30/30 |
| materialize | 1.07x | 2.31x | 1.93x | 1.83x | 3.20x | 2.02x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.02x | 1.60x | 1.19x | 2.02x | 1.78x | 29/30 |
| perft_full_d3 | 0.99x | 1.37x | 1.64x | 1.40x | 2.00x | 1.60x | 30/30 |
| fen_parse | 1.02x | 1.49x | 1.49x | 1.17x | 1.52x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 3.05x | 10.12x | 1.92x | 29.11x | 2.61x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.25x | 3.26x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.98x | 1.89x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.19x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.19x | 1.49x | 4.20x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.43x | 56.78x | 108.14x | 122.47x | 98.16x | 5/5 |
| chess960_materialize | 1.09x | 1.83x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.11x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 42.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 21.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.28x | 2.14x | 1.17x | 2.15x | 2.93x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.42x | 1.15x | 1.85x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.97x | 3.01x | 2.21x | 2.96x | 1.55x | 10/10 |
| replay_trusted_native | 0.95x | 0.83x | 1.62x | 1.55x | 2.14x | 1.94x | 9/10 |
| replay_checked_native | 1.00x | 2.05x | 1.15x | 8.85x | 8.64x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
