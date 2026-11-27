| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.47x | 1.69x | 1.83x | 2.20x | 2.19x | 30/30 |
| materialize | 1.00x | 1.98x | 1.55x | 1.36x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.63x | 1.48x | 2.10x | 2.14x | 30/30 |
| perft_full_d3 | 0.99x | 1.47x | 1.59x | 1.32x | 1.95x | 1.84x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.32x | 1.26x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.32x | 1.61x | 30.18x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.09x | 1.01x | 1.47x | 0.99x | 0.95x | 18/30 |
| san_parse_all | 0.98x | 1.80x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.42x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.01x | 1.19x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.48x | 128.65x | 173.32x | 140.81x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.01x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.99x | 1.87x | 1.48x | 2.20x | 2.45x | 10/10 |
| corpus_materialize | 0.99x | 1.63x | 1.53x | 1.22x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.94x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.66x | 2.02x | 2.24x | 2.30x | 8/10 |
| replay_checked_native | 1.00x | 2.33x | 1.13x | 7.60x | 6.77x | 8.78x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
