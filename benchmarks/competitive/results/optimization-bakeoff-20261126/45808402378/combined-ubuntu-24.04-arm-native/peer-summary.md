| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.90x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.83x | 1.44x | 1.26x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.43x | 1.40x | 1.27x | 1.78x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.27x | 1.06x | 1.56x | 1.48x | 28/30 |
| fen_parse | 0.99x | 1.24x | 1.34x | 1.27x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.43x | 1.59x | 29.94x | 2.40x | 30/30 |
| snapshot | 0.96x | 1.09x | 1.02x | 1.45x | 1.03x | 0.95x | 21/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.78x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.28x | 1.52x | 4.13x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.30x | 76.37x | 131.75x | 176.22x | 143.36x | 5/5 |
| chess960_materialize | 1.04x | 1.58x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.68x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 3.99x | 1.51x | 1.23x | 1.79x | 1.98x | 10/10 |
| corpus_materialize | 0.98x | 1.55x | 1.47x | 1.17x | 1.84x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.90x | 2.08x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.52x | 1.37x | 1.67x | 1.86x | 1.90x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 0.98x | 6.54x | 5.85x | 7.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
