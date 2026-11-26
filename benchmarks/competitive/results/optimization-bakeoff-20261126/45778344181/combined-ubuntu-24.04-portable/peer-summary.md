| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 2.95x | 1.50x | 1.36x | 1.90x | 1.32x | 29/30 |
| materialize | 1.07x | 2.26x | 1.78x | 1.27x | 2.35x | 1.62x | 30/30 |
| perft_bulk_d4 | 0.86x | 2.46x | 1.35x | 1.04x | 1.64x | 1.41x | 27/30 |
| perft_full_d3 | 1.05x | 1.30x | 1.53x | 1.18x | 1.80x | 1.68x | 29/30 |
| fen_parse | 1.00x | 1.75x | 1.60x | 1.28x | 2.05x | 1.34x | 30/30 |
| fen_write_native_ep | 0.99x | 1.86x | 6.58x | 1.20x | 18.35x | 1.41x | 30/30 |
| snapshot | 1.06x | 1.82x | 1.31x | 3.04x | 1.21x | 0.99x | 25/30 |
| san_parse_all | 0.86x | 1.93x | N/A | 3.36x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 9.91x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.73x | 1.39x | 0.86x | 3.26x | 0.80x | 0.82x | 12/30 |
| make_unmake_full | 0.78x | N/A | N/A | 0.99x | N/A | N/A | 2/6 |
| startpos | 1.08x | 1.41x | 78.60x | 115.09x | 176.11x | 116.31x | 5/5 |
| chess960_materialize | 1.11x | 1.78x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 1.98x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 22.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.16x | 1.78x | 1.24x | 1.92x | 1.83x | 10/10 |
| corpus_materialize | 1.00x | 2.00x | 1.84x | 1.31x | 2.90x | 1.67x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.05x | 2.98x | 2.24x | 2.79x | 1.60x | 10/10 |
| replay_trusted_native | 0.83x | 0.57x | 1.15x | 1.24x | 1.87x | 1.65x | 8/10 |
| replay_checked_native | 0.93x | 2.05x | 0.92x | 4.99x | 5.47x | 6.79x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
