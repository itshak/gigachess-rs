| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.00x | 1.48x | 1.42x | 2.16x | 1.79x | 30/30 |
| materialize | 1.04x | 1.87x | 1.24x | 1.06x | 2.17x | 1.43x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.61x | 1.39x | 1.21x | 2.04x | 1.80x | 30/30 |
| perft_full_d3 | 0.95x | 1.68x | 1.29x | 1.12x | 1.64x | 1.69x | 29/30 |
| fen_parse | 1.26x | 1.49x | 1.59x | 1.47x | 2.28x | 1.64x | 30/30 |
| fen_write_native_ep | 0.94x | 3.66x | 7.24x | 1.50x | 29.70x | 1.86x | 30/30 |
| snapshot | 1.00x | 1.78x | 1.16x | 4.07x | 1.16x | 1.18x | 30/30 |
| san_parse_all | 1.04x | 1.96x | N/A | 4.79x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.37x | N/A | 3.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 1.87x | 1.20x | 7.87x | 1.29x | 1.59x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 76.56x | 131.14x | 192.68x | 152.00x | 4/5 |
| chess960_materialize | 1.01x | 1.45x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.43x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.24x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.82x | 1.55x | 1.13x | 2.00x | 1.86x | 9/10 |
| corpus_materialize | 1.02x | 1.73x | 1.31x | 1.10x | 2.67x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.11x | 1.87x | 2.37x | 1.93x | 2.26x | 1.51x | 10/10 |
| replay_trusted_native | 0.99x | 1.02x | 1.85x | 2.29x | 3.28x | 4.11x | 9/10 |
| replay_checked_native | 1.04x | 2.28x | 0.99x | 5.66x | 7.09x | 7.76x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
