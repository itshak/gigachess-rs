| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.99x | 1.43x | 1.38x | 2.15x | 1.72x | 30/30 |
| materialize | 1.03x | 1.87x | 1.61x | 1.19x | 2.16x | 1.46x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.66x | 1.47x | 1.24x | 2.20x | 1.87x | 30/30 |
| perft_full_d3 | 1.00x | 0.81x | 0.93x | 0.81x | 1.18x | 1.16x | 14/30 |
| fen_parse | 0.98x | 1.23x | 1.30x | 1.17x | 2.05x | 1.28x | 30/30 |
| fen_write_native_ep | 0.99x | 2.15x | 4.98x | 0.94x | 14.85x | 1.11x | 25/30 |
| snapshot | 1.00x | 1.34x | 1.14x | 2.04x | 1.14x | 0.87x | 24/30 |
| san_parse_all | 0.98x | 1.93x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.68x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.67x | 0.96x | 3.34x | 1.07x | 1.17x | 25/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.14x | 67.43x | 123.31x | 195.68x | 136.47x | 5/5 |
| chess960_materialize | 1.00x | 1.39x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.38x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 24.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 18.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.21x | 1.75x | 1.32x | 2.56x | 2.31x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.56x | 1.18x | 2.01x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.36x | 1.80x | 1.39x | 1.72x | 1.03x | 10/10 |
| replay_trusted_native | 1.01x | 0.62x | 1.15x | 1.38x | 1.82x | 2.10x | 8/10 |
| replay_checked_native | 0.95x | 1.98x | 0.90x | 6.72x | 6.75x | 8.15x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
