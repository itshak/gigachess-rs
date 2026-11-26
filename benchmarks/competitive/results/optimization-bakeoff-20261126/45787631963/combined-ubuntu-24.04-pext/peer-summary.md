| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.12x | 1.72x | 1.49x | 2.34x | 1.74x | 30/30 |
| materialize | 1.02x | 2.15x | 1.59x | 1.66x | 2.47x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.37x | 1.34x | 1.05x | 2.10x | 1.64x | 28/30 |
| perft_full_d3 | 0.94x | 1.42x | 1.66x | 1.60x | 2.11x | 1.78x | 30/30 |
| fen_parse | 1.02x | 1.38x | 1.55x | 1.14x | 1.45x | 1.22x | 30/30 |
| fen_write_native_ep | 1.01x | 4.72x | 10.08x | 1.90x | 28.19x | 2.52x | 30/30 |
| snapshot | 0.50x | 1.50x | 1.17x | 2.00x | 1.17x | 1.00x | 27/30 |
| san_parse_all | 1.15x | 1.95x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.28x | N/A | 2.22x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.18x | 1.50x | 3.78x | 1.26x | 1.63x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 0.30x | 0.61x | 29.95x | 56.88x | 66.64x | 59.55x | 4/5 |
| chess960_materialize | 1.01x | 1.78x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.34x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 39.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.68x | 4.62x | 1.72x | 0.95x | 2.11x | 1.89x | 8/10 |
| corpus_materialize | 0.96x | 1.39x | 1.29x | 1.06x | 1.70x | 1.13x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.30x | 3.00x | 2.24x | 2.92x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.73x | 1.32x | 1.28x | 1.77x | 2.07x | 8/10 |
| replay_checked_native | 1.01x | 2.02x | 1.07x | 8.45x | 8.22x | 9.52x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
