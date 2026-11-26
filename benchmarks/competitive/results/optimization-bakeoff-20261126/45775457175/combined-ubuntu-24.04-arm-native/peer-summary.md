| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.67x | 1.41x | 1.49x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.40x | 1.21x | 1.95x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.54x | 1.46x | 1.32x | 1.85x | 1.90x | 30/30 |
| perft_full_d3 | 0.99x | 0.82x | 0.89x | 0.76x | 1.08x | 1.04x | 10/30 |
| fen_parse | 1.00x | 1.23x | 1.34x | 1.28x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 3.01x | 6.25x | 1.04x | 19.97x | 1.59x | 28/30 |
| snapshot | 1.00x | 1.18x | 0.95x | 1.50x | 0.92x | 0.96x | 12/30 |
| san_parse_all | 1.00x | 1.79x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.06x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 0.91x | 1.05x | 3.04x | 0.87x | 0.88x | 11/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.15x | 71.19x | 119.59x | 157.91x | 126.73x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.41x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 18.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.53x | 1.74x | 1.38x | 2.06x | 2.25x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.50x | 1.19x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.39x | 1.94x | 1.39x | 1.87x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.40x | 1.07x | 1.29x | 1.45x | 1.48x | 8/10 |
| replay_checked_native | 1.02x | 1.86x | 0.90x | 6.01x | 5.48x | 7.08x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
