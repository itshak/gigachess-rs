| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.78x | 1.48x | 1.55x | 1.88x | 1.88x | 30/30 |
| materialize | 1.00x | 1.81x | 1.43x | 1.24x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.54x | 1.46x | 1.32x | 1.84x | 1.89x | 30/30 |
| perft_full_d3 | 0.97x | 1.14x | 1.24x | 1.02x | 1.51x | 1.43x | 27/30 |
| fen_parse | 0.96x | 1.20x | 1.29x | 1.25x | 1.70x | 1.35x | 30/30 |
| fen_write_native_ep | 0.99x | 4.54x | 9.36x | 1.57x | 30.11x | 2.36x | 30/30 |
| snapshot | 1.18x | 1.40x | 1.27x | 1.86x | 1.29x | 1.18x | 30/30 |
| san_parse_all | 0.96x | 1.77x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 9.91x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.13x | 1.35x | 3.65x | 1.10x | 1.10x | 25/30 |
| make_unmake_full | 0.87x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.21x | 71.11x | 122.24x | 156.58x | 123.47x | 5/5 |
| chess960_materialize | 0.99x | 1.50x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.49x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 26.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 21.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.77x | 1.85x | 1.56x | 2.16x | 2.42x | 10/10 |
| corpus_materialize | 1.02x | 1.57x | 1.51x | 1.22x | 1.87x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.10x | 2.91x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.88x | 0.45x | 1.20x | 1.46x | 1.62x | 1.67x | 8/10 |
| replay_checked_native | 0.92x | 1.85x | 0.90x | 6.12x | 5.41x | 7.07x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
