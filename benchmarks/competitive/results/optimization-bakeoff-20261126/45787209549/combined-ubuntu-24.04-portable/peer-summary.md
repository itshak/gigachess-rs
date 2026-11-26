| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.91x | 1.49x | 1.34x | 1.91x | 1.33x | 29/30 |
| materialize | 0.97x | 2.15x | 1.85x | 1.23x | 2.38x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.87x | 1.54x | 1.21x | 1.92x | 1.65x | 30/30 |
| perft_full_d3 | 0.98x | 1.24x | 1.51x | 1.13x | 1.73x | 1.80x | 30/30 |
| fen_parse | 1.00x | 1.66x | 1.52x | 1.20x | 2.35x | 1.30x | 30/30 |
| fen_write_native_ep | 0.99x | 2.58x | 8.52x | 1.69x | 25.54x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.22x | 2.85x | 1.14x | 0.93x | 24/30 |
| san_parse_all | 0.99x | 2.12x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.40x | N/A | 2.44x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.85x | 1.15x | 4.38x | 1.08x | 1.14x | 25/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 74.10x | 109.94x | 182.33x | 112.06x | 5/5 |
| chess960_materialize | 0.99x | 1.79x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.18x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 21.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 20.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.14x | 1.77x | 1.24x | 1.93x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 1.94x | 1.78x | 1.30x | 2.87x | 1.66x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.06x | 2.95x | 2.24x | 2.83x | 1.61x | 10/10 |
| replay_trusted_native | 0.98x | 0.66x | 1.33x | 1.45x | 2.17x | 1.92x | 8/10 |
| replay_checked_native | 1.01x | 2.26x | 1.00x | 5.47x | 6.02x | 6.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
