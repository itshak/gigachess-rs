| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.74x | 1.75x | 1.68x | 2.61x | 2.13x | 30/30 |
| materialize | 1.00x | 2.01x | 1.35x | 1.14x | 2.35x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.02x | 1.66x | 1.45x | 2.46x | 2.25x | 30/30 |
| perft_full_d3 | 1.02x | 1.50x | 1.55x | 1.25x | 2.10x | 1.86x | 30/30 |
| fen_parse | 1.07x | 1.55x | 1.63x | 1.60x | 2.37x | 1.64x | 30/30 |
| fen_write_native_ep | 0.99x | 4.00x | 7.65x | 1.59x | 30.98x | 2.00x | 30/30 |
| snapshot | 1.00x | 1.80x | 1.29x | 4.13x | 1.26x | 1.19x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.77x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.36x | N/A | 3.35x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.94x | 1.31x | 8.22x | 1.53x | 1.70x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 86.82x | 138.11x | 195.32x | 145.99x | 5/5 |
| chess960_materialize | 1.02x | 1.65x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.88x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.62x | 1.79x | 1.31x | 2.36x | 2.16x | 9/10 |
| corpus_materialize | 1.00x | 1.79x | 1.39x | 1.18x | 2.84x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.87x | 2.39x | 2.06x | 2.26x | 1.52x | 10/10 |
| replay_trusted_native | 0.93x | 1.08x | 1.83x | 2.34x | 3.99x | 5.33x | 9/10 |
| replay_checked_native | 1.00x | 2.45x | 1.08x | 6.46x | 7.84x | 8.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
