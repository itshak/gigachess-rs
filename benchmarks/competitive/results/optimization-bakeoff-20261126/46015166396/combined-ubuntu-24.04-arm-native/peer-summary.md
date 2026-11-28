| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.69x | 1.82x | 2.20x | 2.19x | 30/30 |
| materialize | 1.02x | 1.99x | 1.56x | 1.35x | 2.19x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.63x | 1.49x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.68x | 1.81x | 1.51x | 2.22x | 2.10x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.34x | 1.29x | 1.77x | 1.43x | 30/30 |
| fen_write_native_ep | 0.99x | 4.49x | 9.26x | 1.56x | 30.03x | 2.38x | 30/30 |
| snapshot | 1.00x | 1.08x | 1.04x | 1.47x | 1.02x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 3.99x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.83x | 128.86x | 173.08x | 140.71x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.30x | 1.41x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.88x | 1.87x | 1.50x | 2.18x | 2.42x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.55x | 1.23x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.64x | 2.00x | 2.26x | 2.28x | 8/10 |
| replay_checked_native | 1.02x | 2.34x | 1.15x | 7.69x | 6.89x | 8.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
