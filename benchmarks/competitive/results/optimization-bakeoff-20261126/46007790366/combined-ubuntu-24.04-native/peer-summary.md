| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.85x | 1.92x | 1.70x | 2.66x | 2.10x | 30/30 |
| materialize | 1.02x | 1.95x | 1.70x | 1.26x | 2.28x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.17x | 1.68x | 1.50x | 2.53x | 2.08x | 30/30 |
| perft_full_d3 | 1.23x | 1.64x | 1.89x | 1.64x | 2.41x | 2.34x | 30/30 |
| fen_parse | 1.14x | 1.40x | 1.46x | 1.16x | 2.16x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 4.23x | 9.44x | 1.74x | 28.59x | 2.05x | 30/30 |
| snapshot | 1.01x | 1.99x | 1.25x | 4.06x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.24x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.35x | 1.33x | 4.66x | 1.48x | 1.63x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.16x | 134.92x | 218.53x | 146.63x | 5/5 |
| chess960_materialize | 1.00x | 1.63x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.87x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 24.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.83x | 1.96x | 1.47x | 2.85x | 2.55x | 10/10 |
| corpus_materialize | 1.02x | 1.61x | 1.56x | 1.17x | 2.02x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.41x | 3.18x | 2.45x | 3.06x | 1.74x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.58x | 1.91x | 2.59x | 2.98x | 9/10 |
| replay_checked_native | 1.00x | 2.49x | 1.07x | 8.42x | 8.19x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
