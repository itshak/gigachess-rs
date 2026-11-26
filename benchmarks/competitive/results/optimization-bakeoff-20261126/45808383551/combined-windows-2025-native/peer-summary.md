| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.39x | 1.52x | 1.47x | 2.13x | 1.83x | 30/30 |
| materialize | 0.99x | 1.89x | 1.29x | 1.20x | 2.19x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.63x | 1.44x | 1.27x | 2.04x | 1.84x | 30/30 |
| perft_full_d3 | 1.02x | 1.31x | 1.39x | 1.25x | 1.84x | 1.77x | 30/30 |
| fen_parse | 0.98x | 1.16x | 1.36x | 1.20x | 2.08x | 1.42x | 30/30 |
| fen_write_native_ep | 0.99x | 4.18x | 8.06x | 1.58x | 31.93x | 1.95x | 30/30 |
| snapshot | 0.79x | 1.55x | 1.19x | 2.55x | 1.20x | 1.03x | 30/30 |
| san_parse_all | 1.02x | 1.99x | N/A | 5.36x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.28x | N/A | 3.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.50x | 1.56x | 8.70x | 1.58x | 1.70x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.20x | 75.02x | 144.81x | 224.99x | 169.01x | 5/5 |
| chess960_materialize | 1.07x | 1.55x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.20x | 2.67x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 29.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.75x | 1.61x | 1.16x | 2.20x | 2.16x | 9/10 |
| corpus_materialize | 0.96x | 1.48x | 1.20x | 1.09x | 1.97x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.78x | 2.06x | 1.97x | 2.18x | 1.33x | 10/10 |
| replay_trusted_native | 1.07x | 0.75x | 1.43x | 1.85x | 2.40x | 2.86x | 8/10 |
| replay_checked_native | 0.92x | 2.32x | 0.93x | 7.06x | 7.13x | 8.42x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
