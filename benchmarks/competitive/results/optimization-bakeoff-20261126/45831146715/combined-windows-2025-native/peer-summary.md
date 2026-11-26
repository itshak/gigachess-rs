| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.31x | 1.58x | 1.52x | 2.28x | 1.92x | 30/30 |
| materialize | 0.97x | 1.92x | 1.26x | 1.06x | 2.17x | 1.41x | 27/30 |
| perft_bulk_d4 | 1.05x | 3.89x | 1.50x | 1.33x | 2.29x | 2.07x | 30/30 |
| perft_full_d3 | 0.89x | 1.61x | 1.17x | 0.96x | 1.57x | 1.39x | 26/30 |
| fen_parse | 0.99x | 1.26x | 1.41x | 1.29x | 1.95x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.03x | 7.99x | 1.61x | 31.41x | 2.01x | 30/30 |
| snapshot | 1.02x | 1.78x | 1.26x | 4.16x | 1.26x | 1.13x | 30/30 |
| san_parse_all | 1.01x | 1.97x | N/A | 4.76x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.88x | N/A | 3.23x | N/A | N/A | 12/12 |
| apply_children_full | 0.67x | 1.28x | 0.87x | 5.56x | 1.01x | 1.15x | 20/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 88.00x | 133.29x | 200.34x | 146.73x | 4/5 |
| chess960_materialize | 0.97x | 1.51x | 1.21x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.49x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.08x | 1.63x | 1.20x | 2.16x | 1.96x | 9/10 |
| corpus_materialize | 1.01x | 1.67x | 1.32x | 1.14x | 2.66x | 1.45x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.88x | 2.40x | 1.96x | 2.28x | 1.50x | 10/10 |
| replay_trusted_native | 0.80x | 0.80x | 1.39x | 1.73x | 2.74x | 3.43x | 9/10 |
| replay_checked_native | 1.04x | 2.33x | 0.99x | 5.64x | 6.60x | 8.03x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
