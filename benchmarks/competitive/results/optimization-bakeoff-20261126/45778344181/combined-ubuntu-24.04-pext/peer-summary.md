| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.23x | 1.76x | 1.53x | 2.37x | 1.80x | 30/30 |
| materialize | 1.01x | 2.11x | 1.68x | 1.75x | 2.69x | 1.90x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.73x | 1.48x | 1.16x | 2.30x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.79x | 1.69x | 2.19x | 1.79x | 30/30 |
| fen_parse | 1.01x | 1.38x | 1.24x | 1.09x | 1.38x | 1.21x | 30/30 |
| fen_write_native_ep | 0.96x | 2.86x | 6.15x | 1.26x | 18.03x | 1.55x | 30/30 |
| snapshot | 0.79x | 1.37x | 1.07x | 2.44x | 1.07x | 0.99x | 25/30 |
| san_parse_all | 1.01x | 1.95x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.74x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.68x | 1.18x | 0.74x | 2.20x | 0.74x | 0.93x | 12/30 |
| make_unmake_full | 0.87x | N/A | N/A | 0.99x | N/A | N/A | 1/6 |
| startpos | 1.00x | 1.29x | 80.79x | 143.85x | 172.61x | 155.17x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.18x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 38.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 22.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 6.67x | 2.48x | 1.37x | 3.04x | 2.79x | 10/10 |
| corpus_materialize | 1.02x | 1.48x | 1.34x | 1.13x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.24x | 2.97x | 2.20x | 2.92x | 1.56x | 10/10 |
| replay_trusted_native | 0.98x | 0.69x | 1.26x | 1.26x | 1.75x | 2.03x | 8/10 |
| replay_checked_native | 0.98x | 1.97x | 1.03x | 8.29x | 7.89x | 9.31x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
