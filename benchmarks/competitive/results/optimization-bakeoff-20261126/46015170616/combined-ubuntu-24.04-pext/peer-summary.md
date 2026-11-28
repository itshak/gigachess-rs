| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 5.19x | 1.76x | 1.81x | 2.77x | 2.09x | 30/30 |
| materialize | 1.02x | 2.02x | 1.69x | 1.29x | 2.34x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.24x | 1.64x | 1.47x | 2.58x | 2.01x | 30/30 |
| perft_full_d3 | 1.28x | 1.66x | 1.87x | 1.66x | 2.43x | 2.25x | 30/30 |
| fen_parse | 1.00x | 1.42x | 1.43x | 1.16x | 2.19x | 1.27x | 30/30 |
| fen_write_native_ep | 1.00x | 4.25x | 10.57x | 1.70x | 28.01x | 2.05x | 30/30 |
| snapshot | 1.29x | 1.98x | 1.25x | 5.74x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.12x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.78x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.42x | 1.37x | 4.88x | 1.53x | 1.70x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.18x | 135.92x | 224.70x | 145.07x | 5/5 |
| chess960_materialize | 1.00x | 1.61x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.88x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 28.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.05x | 1.92x | 1.54x | 2.97x | 2.54x | 10/10 |
| corpus_materialize | 0.99x | 1.62x | 1.55x | 1.20x | 2.06x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.42x | 3.20x | 2.45x | 3.05x | 1.80x | 10/10 |
| replay_trusted_native | 0.99x | 0.95x | 1.64x | 1.93x | 2.58x | 2.84x | 9/10 |
| replay_checked_native | 0.93x | 2.37x | 1.06x | 7.97x | 7.86x | 9.07x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
