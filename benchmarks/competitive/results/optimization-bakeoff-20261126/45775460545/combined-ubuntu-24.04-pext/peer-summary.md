| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.15x | 1.75x | 1.52x | 2.33x | 1.77x | 30/30 |
| materialize | 1.00x | 2.07x | 1.60x | 1.71x | 2.58x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.93x | 1.56x | 1.25x | 2.44x | 2.03x | 30/30 |
| perft_full_d3 | 1.10x | 0.93x | 1.04x | 1.05x | 1.39x | 1.12x | 17/30 |
| fen_parse | 0.90x | 1.17x | 1.30x | 1.14x | 1.48x | 1.06x | 29/30 |
| fen_write_native_ep | 0.96x | 3.14x | 6.56x | 1.19x | 19.18x | 1.34x | 30/30 |
| snapshot | 1.67x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.86x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.42x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.61x | 1.01x | 3.04x | 1.02x | 1.27x | 24/30 |
| make_unmake_full | 1.04x | N/A | N/A | 0.97x | N/A | N/A | 1/6 |
| startpos | 0.83x | 0.83x | 65.32x | 120.72x | 153.42x | 116.30x | 4/5 |
| chess960_materialize | 0.99x | 1.63x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.27x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 36.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 18.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.60x | 2.43x | 1.39x | 3.04x | 2.76x | 10/10 |
| corpus_materialize | 1.01x | 1.45x | 1.34x | 1.12x | 1.76x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.43x | 1.91x | 1.42x | 1.87x | 0.96x | 8/10 |
| replay_trusted_native | 1.05x | 0.63x | 1.12x | 1.16x | 1.62x | 1.84x | 7/10 |
| replay_checked_native | 1.07x | 1.95x | 1.02x | 8.28x | 7.92x | 9.34x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
