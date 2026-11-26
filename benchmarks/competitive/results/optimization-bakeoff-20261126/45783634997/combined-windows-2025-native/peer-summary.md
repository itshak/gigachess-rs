| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.26x | 1.49x | 1.44x | 2.09x | 1.85x | 30/30 |
| materialize | 1.02x | 1.94x | 1.34x | 1.21x | 2.20x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.54x | 1.37x | 2.21x | 1.96x | 30/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.43x | 1.26x | 1.87x | 1.79x | 30/30 |
| fen_parse | 1.12x | 1.33x | 1.41x | 1.33x | 2.16x | 1.41x | 30/30 |
| fen_write_native_ep | 1.61x | 4.36x | 8.60x | 1.56x | 33.86x | 2.02x | 30/30 |
| snapshot | 1.03x | 1.26x | 1.14x | 1.86x | 1.14x | 0.86x | 24/30 |
| san_parse_all | 1.06x | 2.06x | N/A | 5.67x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.78x | N/A | 3.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.99x | 1.20x | 8.25x | 1.29x | 1.43x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.85x | 1.01x | 60.18x | 119.64x | 171.21x | 123.41x | 5/5 |
| chess960_materialize | 0.95x | 1.47x | 1.18x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.47x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 19.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.34x | 1.82x | 1.48x | 2.47x | 2.37x | 10/10 |
| corpus_materialize | 0.98x | 1.60x | 1.34x | 1.16x | 1.98x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.78x | 2.33x | 1.94x | 2.17x | 1.40x | 10/10 |
| replay_trusted_native | 0.97x | 0.65x | 1.27x | 1.63x | 2.08x | 2.48x | 8/10 |
| replay_checked_native | 0.99x | 2.26x | 0.95x | 7.55x | 7.44x | 9.34x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
