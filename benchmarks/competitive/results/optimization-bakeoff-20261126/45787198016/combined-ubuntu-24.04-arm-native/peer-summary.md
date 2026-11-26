| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.76x | 1.43x | 1.55x | 1.88x | 1.85x | 30/30 |
| materialize | 1.00x | 1.83x | 1.43x | 1.25x | 2.01x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.45x | 1.41x | 1.29x | 1.80x | 1.85x | 30/30 |
| perft_full_d3 | 0.92x | 1.07x | 1.16x | 0.98x | 1.41x | 1.35x | 25/30 |
| fen_parse | 0.97x | 1.22x | 1.31x | 1.27x | 1.74x | 1.38x | 30/30 |
| fen_write_native_ep | 0.99x | 4.44x | 9.17x | 1.60x | 30.20x | 2.34x | 30/30 |
| snapshot | 1.25x | 1.49x | 1.34x | 1.98x | 1.28x | 1.30x | 30/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.75x | 8.92x | N/A | 1.91x | N/A | N/A | 12/12 |
| apply_children_full | 0.79x | 0.90x | 1.06x | 2.88x | 0.86x | 0.87x | 13/30 |
| make_unmake_full | 0.78x | N/A | N/A | 1.12x | N/A | N/A | 5/6 |
| startpos | 0.93x | 1.16x | 70.89x | 120.02x | 161.07x | 129.23x | 5/5 |
| chess960_materialize | 0.99x | 1.49x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.44x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 28.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.22x | 23.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.80x | 1.84x | 1.44x | 2.13x | 2.37x | 10/10 |
| corpus_materialize | 1.01x | 1.59x | 1.51x | 1.21x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.10x | 2.83x | 1.46x | 10/10 |
| replay_trusted_native | 0.80x | 0.41x | 1.07x | 1.29x | 1.42x | 1.46x | 7/10 |
| replay_checked_native | 0.80x | 1.63x | 0.80x | 5.31x | 4.76x | 6.18x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
