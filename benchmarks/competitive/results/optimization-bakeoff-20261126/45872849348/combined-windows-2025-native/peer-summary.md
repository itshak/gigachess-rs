| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 4.74x | 1.75x | 1.68x | 2.89x | 2.12x | 30/30 |
| materialize | 1.03x | 2.02x | 1.36x | 1.14x | 2.40x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.64x | 1.48x | 1.31x | 2.35x | 2.09x | 30/30 |
| perft_full_d3 | 0.99x | 1.48x | 1.54x | 1.29x | 2.09x | 1.84x | 30/30 |
| fen_parse | 0.97x | 1.39x | 1.46x | 1.37x | 2.17x | 1.45x | 30/30 |
| fen_write_native_ep | 1.01x | 3.95x | 7.65x | 1.59x | 31.38x | 1.99x | 30/30 |
| snapshot | 0.98x | 1.75x | 1.22x | 4.03x | 1.22x | 1.12x | 30/30 |
| san_parse_all | 1.02x | 1.99x | N/A | 4.80x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.61x | N/A | 3.44x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.69x | 1.14x | 7.13x | 1.34x | 1.49x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 84.33x | 132.99x | 203.09x | 143.19x | 5/5 |
| chess960_materialize | 1.00x | 1.64x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.74x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.35x | 1.77x | 1.30x | 2.37x | 2.13x | 9/10 |
| corpus_materialize | 0.99x | 1.77x | 1.41x | 1.18x | 2.82x | 1.51x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.87x | 2.37x | 1.97x | 2.28x | 1.50x | 10/10 |
| replay_trusted_native | 0.84x | 0.85x | 1.47x | 1.82x | 2.89x | 3.61x | 8/10 |
| replay_checked_native | 1.00x | 2.46x | 1.09x | 6.13x | 7.45x | 8.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
