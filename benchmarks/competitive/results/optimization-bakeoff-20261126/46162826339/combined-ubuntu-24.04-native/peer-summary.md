| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.00x | 1.75x | 1.66x | 2.44x | 2.12x | 30/30 |
| materialize | 0.98x | 2.13x | 1.64x | 1.20x | 2.35x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.46x | 1.70x | 1.34x | 2.56x | 2.43x | 30/30 |
| perft_full_d3 | 1.21x | 1.68x | 2.01x | 1.50x | 2.55x | 2.50x | 30/30 |
| fen_parse | 0.96x | 1.34x | 1.50x | 1.22x | 2.00x | 1.36x | 30/30 |
| fen_write_native_ep | 0.99x | 3.85x | 8.52x | 1.55x | 25.40x | 1.91x | 30/30 |
| snapshot | 1.00x | 2.49x | 1.50x | 8.19x | 1.50x | 1.50x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.89x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.93x | 1.41x | 4.10x | 1.38x | 1.92x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 84.44x | 144.49x | 220.87x | 157.03x | 5/5 |
| chess960_materialize | 0.99x | 1.73x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.81x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 34.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.60x | 1.56x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 27.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.26x | 1.88x | 1.25x | 2.49x | 2.32x | 9/10 |
| corpus_materialize | 1.07x | 1.65x | 1.54x | 1.13x | 2.23x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.26x | 2.94x | 2.36x | 2.91x | 1.64x | 10/10 |
| replay_trusted_native | 1.03x | 0.77x | 1.47x | 1.64x | 2.72x | 3.59x | 9/10 |
| replay_checked_native | 1.01x | 2.76x | 1.18x | 9.01x | 9.20x | 11.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
