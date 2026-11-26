| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.65x | 1.40x | 1.52x | 1.82x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.39x | 1.22x | 1.95x | 1.35x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.36x | 1.38x | 1.24x | 1.76x | 1.80x | 30/30 |
| perft_full_d3 | 0.93x | 0.76x | 0.83x | 0.70x | 1.01x | 0.96x | 5/30 |
| fen_parse | 0.90x | 1.13x | 1.21x | 1.15x | 1.60x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 3.04x | 6.18x | 1.06x | 19.63x | 1.60x | 29/30 |
| snapshot | 0.99x | 1.23x | 0.91x | 1.56x | 0.87x | 0.99x | 12/30 |
| san_parse_all | 1.01x | 1.78x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 10.00x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 0.79x | 0.91x | 2.58x | 0.76x | 0.77x | 7/30 |
| make_unmake_full | 0.82x | N/A | N/A | 0.95x | N/A | N/A | 1/6 |
| startpos | 0.57x | 0.67x | 43.19x | 74.58x | 98.65x | 77.64x | 4/5 |
| chess960_materialize | 1.00x | 1.42x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.26x | 1.15x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.80x | 20.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.80x | 15.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.48x | 1.75x | 1.36x | 2.05x | 2.24x | 10/10 |
| corpus_materialize | 0.99x | 1.56x | 1.49x | 1.18x | 1.86x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.42x | 1.93x | 1.39x | 1.90x | 0.97x | 8/10 |
| replay_trusted_native | 0.85x | 0.35x | 0.92x | 1.12x | 1.24x | 1.27x | 6/10 |
| replay_checked_native | 0.88x | 1.64x | 0.80x | 5.36x | 4.80x | 6.17x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
