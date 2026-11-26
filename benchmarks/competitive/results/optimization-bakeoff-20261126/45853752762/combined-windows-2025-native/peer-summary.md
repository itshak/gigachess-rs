| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.98x | 1.75x | 1.66x | 2.48x | 2.13x | 30/30 |
| materialize | 0.94x | 2.08x | 1.35x | 1.29x | 2.47x | 1.55x | 29/30 |
| perft_bulk_d4 | 0.97x | 4.16x | 1.61x | 1.39x | 2.35x | 2.08x | 30/30 |
| perft_full_d3 | 0.96x | 1.40x | 1.48x | 1.34x | 1.91x | 1.86x | 30/30 |
| fen_parse | 0.98x | 1.32x | 1.46x | 1.33x | 2.14x | 1.41x | 30/30 |
| fen_write_native_ep | 0.95x | 3.90x | 8.27x | 1.57x | 31.57x | 2.03x | 30/30 |
| snapshot | 0.95x | 1.90x | 1.00x | 3.91x | 1.01x | 0.96x | 23/30 |
| san_parse_all | 0.96x | 2.08x | N/A | 5.47x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.41x | N/A | 3.75x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.04x | 1.30x | 8.56x | 1.34x | 1.50x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.09x | 157.00x | 228.57x | 169.32x | 5/5 |
| chess960_materialize | 1.02x | 1.68x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.93x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 28.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 21.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.57x | 1.81x | 1.38x | 2.48x | 2.43x | 10/10 |
| corpus_materialize | 1.02x | 1.60x | 1.26x | 1.18x | 1.99x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.67x | 2.16x | 1.72x | 2.07x | 1.33x | 10/10 |
| replay_trusted_native | 0.92x | 0.76x | 1.45x | 1.89x | 2.35x | 2.65x | 8/10 |
| replay_checked_native | 0.96x | 2.23x | 0.97x | 7.52x | 7.37x | 8.74x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
