| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.30x | 1.86x | 1.57x | 2.41x | 1.89x | 30/30 |
| materialize | 0.99x | 2.21x | 1.66x | 1.71x | 2.59x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.53x | 1.17x | 2.30x | 1.84x | 29/30 |
| perft_full_d3 | 1.24x | 1.67x | 1.92x | 1.88x | 2.44x | 2.06x | 30/30 |
| fen_parse | 0.93x | 1.26x | 1.30x | 1.01x | 1.50x | 1.06x | 27/30 |
| fen_write_native_ep | 1.00x | 4.54x | 10.45x | 2.13x | 29.82x | 2.63x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.06x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.35x | 4.06x | 1.35x | 1.70x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.11x | 87.66x | 152.76x | 199.85x | 152.83x | 5/5 |
| chess960_materialize | 0.99x | 1.79x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.46x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 39.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.36x | 2.00x | 1.10x | 2.42x | 2.20x | 9/10 |
| corpus_materialize | 1.01x | 1.40x | 1.28x | 1.05x | 1.69x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.23x | 3.03x | 2.25x | 2.96x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.50x | 1.51x | 2.16x | 2.51x | 9/10 |
| replay_checked_native | 1.02x | 2.20x | 1.14x | 9.19x | 9.27x | 10.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
