| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.24x | 1.62x | 1.74x | 2.10x | 2.10x | 30/30 |
| materialize | 0.98x | 1.92x | 1.52x | 1.33x | 2.12x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.76x | 1.55x | 1.40x | 1.96x | 2.01x | 30/30 |
| perft_full_d3 | 0.99x | 1.49x | 1.59x | 1.33x | 1.95x | 1.84x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.33x | 1.25x | 1.75x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.47x | 9.42x | 1.58x | 29.83x | 2.40x | 30/30 |
| snapshot | 1.02x | 1.17x | 1.01x | 1.49x | 1.00x | 0.94x | 18/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.05x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.43x | 128.31x | 174.09x | 140.61x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.96x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 4.14x | 1.58x | 1.25x | 1.83x | 2.02x | 10/10 |
| corpus_materialize | 0.98x | 1.60x | 1.50x | 1.20x | 1.87x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.64x | 1.66x | 2.01x | 2.25x | 2.29x | 8/10 |
| replay_checked_native | 1.00x | 2.35x | 1.14x | 7.63x | 6.78x | 8.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
