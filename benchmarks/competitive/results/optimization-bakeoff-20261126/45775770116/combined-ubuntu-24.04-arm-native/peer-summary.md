| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.65x | 1.39x | 1.49x | 1.81x | 1.81x | 30/30 |
| materialize | 1.00x | 1.76x | 1.39x | 1.21x | 1.95x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.60x | 1.46x | 1.33x | 1.86x | 1.91x | 30/30 |
| perft_full_d3 | 0.99x | 0.83x | 0.89x | 0.76x | 1.09x | 1.03x | 10/30 |
| fen_parse | 0.99x | 1.24x | 1.33x | 1.26x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 2.96x | 6.13x | 1.08x | 19.19x | 1.57x | 29/30 |
| snapshot | 1.03x | 1.21x | 0.94x | 1.56x | 0.88x | 1.02x | 18/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.10x | N/A | 2.38x | N/A | N/A | 12/12 |
| apply_children_full | 1.16x | 1.04x | 1.18x | 3.35x | 0.99x | 1.01x | 23/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.18x | 75.93x | 123.58x | 178.29x | 144.42x | 5/5 |
| chess960_materialize | 1.00x | 1.40x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.42x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.58x | 1.76x | 1.42x | 2.06x | 2.30x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.49x | 1.20x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.40x | 1.95x | 1.39x | 1.86x | 0.97x | 8/10 |
| replay_trusted_native | 1.04x | 0.42x | 1.12x | 1.36x | 1.50x | 1.55x | 8/10 |
| replay_checked_native | 0.99x | 1.83x | 0.89x | 6.02x | 5.36x | 7.00x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
