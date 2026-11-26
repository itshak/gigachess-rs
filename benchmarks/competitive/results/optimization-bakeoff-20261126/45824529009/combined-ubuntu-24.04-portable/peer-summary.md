| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.93x | 1.49x | 1.35x | 1.92x | 1.34x | 30/30 |
| materialize | 1.02x | 2.00x | 1.76x | 1.27x | 2.36x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.77x | 1.48x | 1.15x | 1.87x | 1.61x | 30/30 |
| perft_full_d3 | 1.00x | 1.42x | 1.63x | 1.30x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.05x | 1.59x | 1.55x | 1.23x | 2.26x | 1.32x | 30/30 |
| fen_write_native_ep | 1.01x | 2.95x | 9.39x | 1.68x | 27.14x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.22x | 3.05x | 1.02x | 0.98x | 24/30 |
| san_parse_all | 1.00x | 2.03x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.53x | N/A | 2.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.20x | 1.54x | 4.46x | 1.41x | 1.46x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 64.37x | 114.32x | 178.73x | 110.05x | 5/5 |
| chess960_materialize | 1.00x | 1.53x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.16x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 27.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 17.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.59x | 1.81x | 1.19x | 2.05x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.54x | 1.13x | 1.99x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.18x | 3.19x | 2.39x | 3.03x | 1.78x | 10/10 |
| replay_trusted_native | 1.04x | 0.95x | 1.67x | 1.98x | 2.61x | 2.41x | 9/10 |
| replay_checked_native | 1.05x | 2.16x | 1.07x | 7.51x | 7.77x | 8.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
