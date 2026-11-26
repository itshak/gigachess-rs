| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.33x | 1.45x | 1.50x | 1.92x | 1.76x | 30/30 |
| materialize | 0.98x | 1.94x | 1.48x | 1.02x | 2.29x | 1.37x | 27/30 |
| perft_bulk_d4 | 1.03x | 3.80x | 1.37x | 1.14x | 1.76x | 1.67x | 29/30 |
| perft_full_d3 | 0.99x | 1.54x | 1.74x | 1.43x | 1.96x | 2.16x | 30/30 |
| fen_parse | 0.92x | 1.19x | 1.23x | 1.23x | 1.56x | 1.28x | 30/30 |
| fen_write_native_ep | 0.98x | 4.14x | 8.98x | 1.76x | 27.81x | 2.21x | 30/30 |
| snapshot | 0.99x | 1.68x | 1.13x | 2.42x | 1.14x | 1.01x | 27/30 |
| san_parse_all | 1.00x | 1.72x | N/A | 3.62x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.54x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.13x | 1.21x | 5.71x | 1.18x | 1.30x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 5/6 |
| startpos | 0.95x | 1.29x | 68.91x | 131.49x | 161.17x | 130.38x | 5/5 |
| chess960_materialize | 0.98x | 1.60x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.73x | 1.25x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.13x | 1.66x | 1.03x | 1.99x | 2.16x | 9/10 |
| corpus_materialize | 0.95x | 1.45x | 1.34x | 1.07x | 1.86x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.82x | 2.56x | 1.88x | 2.45x | 1.40x | 10/10 |
| replay_trusted_native | 0.99x | 0.59x | 1.57x | 1.57x | 1.75x | 1.68x | 8/10 |
| replay_checked_native | 0.99x | 2.31x | 1.06x | 8.51x | 7.35x | 8.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
