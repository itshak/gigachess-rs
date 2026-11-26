| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.00x | 1.68x | 1.46x | 2.44x | 1.71x | 30/30 |
| materialize | 0.91x | 2.02x | 1.50x | 1.68x | 2.64x | 1.96x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.33x | 1.33x | 1.04x | 2.07x | 1.62x | 28/30 |
| perft_full_d3 | 0.95x | 1.55x | 1.72x | 1.67x | 2.18x | 1.78x | 30/30 |
| fen_parse | 0.96x | 1.23x | 1.32x | 1.11x | 1.45x | 1.10x | 30/30 |
| fen_write_native_ep | 0.96x | 4.71x | 10.21x | 1.85x | 26.95x | 2.07x | 30/30 |
| snapshot | 0.65x | 1.50x | 1.16x | 2.00x | 1.16x | 1.00x | 26/30 |
| san_parse_all | 0.90x | 1.90x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.48x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.92x | 1.29x | 3.59x | 1.21x | 2.13x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.33x | 68.04x | 122.02x | 143.21x | 113.84x | 5/5 |
| chess960_materialize | 1.01x | 1.75x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.37x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.10x | 21.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.47x | 1.66x | 0.92x | 2.05x | 1.86x | 8/10 |
| corpus_materialize | 0.95x | 1.39x | 1.26x | 1.05x | 1.72x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 2.97x | 2.22x | 2.91x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.81x | 1.50x | 1.47x | 2.02x | 2.32x | 9/10 |
| replay_checked_native | 0.96x | 1.99x | 1.11x | 8.42x | 8.18x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
