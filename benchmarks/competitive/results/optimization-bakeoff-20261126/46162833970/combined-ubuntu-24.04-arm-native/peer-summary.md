| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.44x | 1.68x | 1.81x | 2.19x | 2.18x | 30/30 |
| materialize | 1.01x | 1.98x | 1.56x | 1.36x | 2.19x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.02x | 1.63x | 1.49x | 2.09x | 2.14x | 30/30 |
| perft_full_d3 | 1.12x | 1.66x | 1.82x | 1.49x | 2.19x | 2.08x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.35x | 1.27x | 1.78x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.30x | 1.58x | 30.19x | 2.46x | 30/30 |
| snapshot | 1.00x | 1.04x | 1.10x | 1.44x | 1.11x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.45x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.49x | 4.03x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.59x | 128.04x | 173.62x | 139.78x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.01x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.37x | 1.48x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.63x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.85x | 1.86x | 1.50x | 2.26x | 2.43x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.54x | 1.23x | 1.94x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.08x | 2.88x | 2.07x | 2.82x | 1.44x | 10/10 |
| replay_trusted_native | 1.02x | 0.64x | 1.69x | 2.04x | 2.29x | 2.34x | 8/10 |
| replay_checked_native | 1.02x | 2.35x | 1.15x | 7.68x | 6.92x | 8.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
