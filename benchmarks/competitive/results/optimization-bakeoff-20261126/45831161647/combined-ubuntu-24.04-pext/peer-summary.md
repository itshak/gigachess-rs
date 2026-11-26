| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.05x | 1.70x | 1.48x | 2.29x | 1.72x | 30/30 |
| materialize | 1.00x | 2.13x | 1.59x | 1.71x | 2.54x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.34x | 1.33x | 1.05x | 2.05x | 1.63x | 28/30 |
| perft_full_d3 | 1.02x | 1.46x | 1.73x | 1.64x | 2.14x | 1.76x | 30/30 |
| fen_parse | 1.07x | 1.39x | 1.36x | 1.13x | 1.46x | 1.24x | 30/30 |
| fen_write_native_ep | 1.04x | 4.92x | 10.40x | 1.91x | 28.17x | 2.50x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.04x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.14x | 1.35x | 4.09x | 1.35x | 1.69x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.11x | 88.09x | 162.67x | 196.46x | 176.44x | 5/5 |
| chess960_materialize | 1.05x | 1.64x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.30x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.63x | 1.72x | 0.96x | 2.10x | 1.90x | 8/10 |
| corpus_materialize | 0.99x | 1.37x | 1.27x | 1.05x | 1.67x | 1.13x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.24x | 2.90x | 2.17x | 2.84x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 0.91x | 1.62x | 1.59x | 2.25x | 2.57x | 9/10 |
| replay_checked_native | 0.94x | 1.94x | 1.05x | 8.11x | 7.80x | 9.07x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
