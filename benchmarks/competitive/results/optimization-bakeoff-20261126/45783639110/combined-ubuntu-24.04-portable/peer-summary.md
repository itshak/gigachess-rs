| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.64x | 1.55x | 1.32x | 1.85x | 1.30x | 29/30 |
| materialize | 1.08x | 2.17x | 1.77x | 1.79x | 2.78x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.98x | 1.58x | 1.17x | 2.02x | 1.79x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.78x | 1.60x | 2.14x | 1.82x | 30/30 |
| fen_parse | 0.90x | 1.43x | 1.31x | 1.12x | 1.38x | 1.18x | 30/30 |
| fen_write_native_ep | 1.01x | 1.96x | 6.52x | 1.17x | 18.08x | 1.71x | 30/30 |
| snapshot | 1.00x | 1.46x | 1.12x | 3.02x | 1.12x | 1.00x | 28/30 |
| san_parse_all | 0.93x | 1.75x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.40x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.84x | 1.18x | 3.45x | 1.11x | 1.29x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 62.96x | 108.47x | 126.06x | 114.48x | 5/5 |
| chess960_materialize | 0.99x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.88x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 41.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 24.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.61x | 2.34x | 1.24x | 2.34x | 3.23x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.43x | 1.17x | 1.87x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.99x | 2.20x | 2.92x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.72x | 1.35x | 1.35x | 1.79x | 1.68x | 8/10 |
| replay_checked_native | 1.00x | 1.95x | 1.14x | 8.57x | 8.35x | 9.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
