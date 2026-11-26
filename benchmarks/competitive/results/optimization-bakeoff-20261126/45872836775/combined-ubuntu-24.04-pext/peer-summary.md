| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.51x | 1.88x | 1.65x | 2.58x | 1.91x | 30/30 |
| materialize | 0.99x | 2.25x | 1.78x | 1.76x | 2.89x | 1.97x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.85x | 1.54x | 1.21x | 2.40x | 1.88x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.56x | 1.50x | 1.99x | 1.68x | 30/30 |
| fen_parse | 0.98x | 1.39x | 1.34x | 1.13x | 1.44x | 1.11x | 30/30 |
| fen_write_native_ep | 0.99x | 4.57x | 10.11x | 2.08x | 27.78x | 2.49x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 7.59x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.01x | 1.89x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.87x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.06x | 1.44x | 1.66x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 82.32x | 149.08x | 173.11x | 142.77x | 4/5 |
| chess960_materialize | 0.94x | 1.65x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.82x | 0.81x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.80x | 18.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.14x | 1.90x | 1.07x | 2.36x | 2.11x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.29x | 1.09x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.23x | 2.89x | 2.19x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 1.02x | 0.93x | 1.70x | 1.64x | 2.32x | 2.63x | 9/10 |
| replay_checked_native | 0.98x | 2.13x | 1.18x | 9.31x | 8.80x | 10.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
