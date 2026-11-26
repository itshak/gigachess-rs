| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 2.83x | 1.59x | 1.38x | 1.83x | 1.33x | 29/30 |
| materialize | 0.95x | 2.29x | 1.76x | 1.74x | 2.67x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.95x | 2.88x | 1.54x | 1.13x | 1.89x | 1.70x | 28/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.62x | 1.40x | 1.84x | 1.61x | 30/30 |
| fen_parse | 1.09x | 1.46x | 1.45x | 1.14x | 1.50x | 1.16x | 30/30 |
| fen_write_native_ep | 1.00x | 3.05x | 10.06x | 1.87x | 28.84x | 2.49x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.02x | 1.95x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.39x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.46x | 4.18x | 1.39x | 1.56x | 30/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 56.97x | 108.41x | 121.82x | 97.09x | 5/5 |
| chess960_materialize | 0.93x | 1.71x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 39.22x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.96x | 19.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 3.75x | 1.89x | 1.03x | 1.91x | 2.58x | 10/10 |
| corpus_materialize | 0.96x | 1.42x | 1.34x | 1.09x | 1.75x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.94x | 2.97x | 2.18x | 2.93x | 1.53x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.70x | 1.62x | 2.22x | 2.03x | 9/10 |
| replay_checked_native | 1.01x | 2.05x | 1.16x | 8.81x | 8.70x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
