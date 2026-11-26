| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.82x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.80x | 1.41x | 1.24x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.52x | 1.39x | 1.95x | 1.99x | 30/30 |
| perft_full_d3 | 1.02x | 1.19x | 1.30x | 1.08x | 1.59x | 1.50x | 29/30 |
| fen_parse | 1.01x | 1.26x | 1.36x | 1.28x | 1.79x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 3.01x | 6.16x | 1.06x | 19.46x | 1.58x | 29/30 |
| snapshot | 1.08x | 1.28x | 0.93x | 1.62x | 0.91x | 1.00x | 17/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.90x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.12x | 1.30x | 3.65x | 1.08x | 1.09x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.19x | 76.06x | 129.48x | 174.01x | 138.20x | 5/5 |
| chess960_materialize | 0.98x | 1.48x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.60x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.65x | 1.83x | 1.46x | 2.12x | 2.34x | 10/10 |
| corpus_materialize | 0.99x | 1.54x | 1.50x | 1.19x | 1.86x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.93x | 2.08x | 2.78x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.49x | 1.30x | 1.57x | 1.74x | 1.79x | 8/10 |
| replay_checked_native | 0.98x | 1.97x | 0.98x | 6.52x | 5.79x | 7.54x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
