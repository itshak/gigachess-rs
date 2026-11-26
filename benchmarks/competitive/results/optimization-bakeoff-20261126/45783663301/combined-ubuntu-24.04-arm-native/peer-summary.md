| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.01x | 1.82x | 1.43x | 1.25x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.52x | 1.38x | 1.94x | 2.00x | 30/30 |
| perft_full_d3 | 0.99x | 1.15x | 1.24x | 1.05x | 1.50x | 1.46x | 27/30 |
| fen_parse | 0.99x | 1.24x | 1.32x | 1.28x | 1.76x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 2.98x | 6.09x | 1.03x | 19.53x | 1.57x | 28/30 |
| snapshot | 1.05x | 1.15x | 1.10x | 1.59x | 1.13x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.80x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.12x | 1.33x | 3.60x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.16x | 66.97x | 112.16x | 148.49x | 118.09x | 5/5 |
| chess960_materialize | 1.02x | 1.53x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.62x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 18.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.79x | 1.91x | 1.50x | 2.18x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.53x | 1.21x | 1.90x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.10x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.31x | 1.61x | 1.78x | 1.83x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 0.99x | 6.75x | 5.96x | 7.70x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
