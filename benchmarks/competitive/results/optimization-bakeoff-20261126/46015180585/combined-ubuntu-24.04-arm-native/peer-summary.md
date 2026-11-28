| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.43x | 1.68x | 1.81x | 2.19x | 2.18x | 30/30 |
| materialize | 1.01x | 1.98x | 1.56x | 1.35x | 2.17x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.02x | 1.64x | 1.48x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.68x | 1.81x | 1.51x | 2.23x | 2.10x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.35x | 1.28x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 0.98x | 4.47x | 9.17x | 1.56x | 29.71x | 2.36x | 30/30 |
| snapshot | 1.01x | 1.10x | 1.03x | 1.47x | 1.03x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.42x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.00x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.47x | 129.84x | 170.38x | 137.98x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.01x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.36x | 1.48x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.92x | 1.87x | 1.50x | 2.17x | 2.42x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.52x | 1.22x | 1.89x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.89x | 2.09x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.66x | 2.01x | 2.26x | 2.29x | 8/10 |
| replay_checked_native | 1.02x | 2.37x | 1.15x | 7.79x | 6.84x | 8.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
