| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.89x | 1.65x | 1.66x | 2.43x | 1.98x | 30/30 |
| materialize | 1.05x | 2.21x | 1.68x | 1.23x | 2.49x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.09x | 4.28x | 1.61x | 1.31x | 2.48x | 2.26x | 30/30 |
| perft_full_d3 | 0.99x | 1.35x | 1.59x | 1.19x | 2.06x | 1.96x | 29/30 |
| fen_parse | 0.99x | 1.17x | 1.33x | 1.04x | 1.81x | 1.12x | 28/30 |
| fen_write_native_ep | 0.99x | 3.91x | 8.64x | 1.45x | 25.16x | 1.97x | 30/30 |
| snapshot | 1.00x | 2.49x | 1.49x | 4.47x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 1.80x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.77x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.35x | 1.86x | 1.38x | 3.92x | 1.33x | 1.86x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 86.54x | 146.66x | 220.49x | 149.90x | 5/5 |
| chess960_materialize | 1.06x | 1.77x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.94x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 35.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 21.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 6.10x | 1.77x | 1.19x | 2.32x | 2.16x | 9/10 |
| corpus_materialize | 1.09x | 1.70x | 1.56x | 1.14x | 2.30x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.06x | 2.34x | 2.98x | 2.46x | 3.03x | 1.72x | 10/10 |
| replay_trusted_native | 0.97x | 0.67x | 1.28x | 1.44x | 2.26x | 2.64x | 8/10 |
| replay_checked_native | 1.02x | 2.43x | 1.13x | 8.49x | 8.67x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
