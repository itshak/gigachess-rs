| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.67x | 1.40x | 1.50x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.78x | 1.40x | 1.21x | 1.96x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.56x | 1.45x | 1.32x | 1.86x | 1.90x | 30/30 |
| perft_full_d3 | 0.99x | 0.82x | 0.89x | 0.75x | 1.09x | 1.04x | 10/30 |
| fen_parse | 0.99x | 1.23x | 1.33x | 1.26x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 3.03x | 6.23x | 1.05x | 19.86x | 1.59x | 28/30 |
| snapshot | 1.00x | 1.18x | 0.95x | 1.50x | 0.93x | 0.96x | 12/30 |
| san_parse_all | 1.00x | 1.77x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.15x | N/A | 2.40x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 0.89x | 1.04x | 2.98x | 0.87x | 0.87x | 11/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.14x | 70.80x | 119.16x | 160.80x | 129.20x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.44x | 1.22x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 24.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.60x | 1.77x | 1.42x | 2.07x | 2.29x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.50x | 1.19x | 1.87x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.94x | 1.39x | 1.88x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.42x | 1.07x | 1.30x | 1.46x | 1.49x | 8/10 |
| replay_checked_native | 1.00x | 1.85x | 0.90x | 6.01x | 5.40x | 7.01x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
