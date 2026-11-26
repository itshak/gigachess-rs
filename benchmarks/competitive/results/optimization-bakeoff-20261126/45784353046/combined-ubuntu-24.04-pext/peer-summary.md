| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.79x | 1.54x | 2.36x | 1.83x | 30/30 |
| materialize | 1.00x | 2.25x | 1.69x | 1.75x | 2.76x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.07x | 1.62x | 1.26x | 2.49x | 1.98x | 30/30 |
| perft_full_d3 | 1.02x | 1.57x | 1.76x | 1.70x | 2.21x | 1.81x | 30/30 |
| fen_parse | 1.07x | 1.34x | 1.38x | 1.07x | 1.49x | 1.13x | 30/30 |
| fen_write_native_ep | 0.96x | 4.44x | 9.97x | 2.06x | 28.33x | 2.55x | 30/30 |
| snapshot | 0.85x | 1.20x | 1.00x | 2.60x | 1.00x | 1.00x | 21/30 |
| san_parse_all | 1.04x | 2.01x | N/A | 4.64x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.87x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.73x | 1.09x | 3.08x | 1.12x | 1.34x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.17x | 66.43x | 114.71x | 148.40x | 118.96x | 5/5 |
| chess960_materialize | 0.93x | 1.65x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.37x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.85x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 22.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.84x | 2.54x | 1.43x | 3.05x | 2.83x | 10/10 |
| corpus_materialize | 0.99x | 1.47x | 1.33x | 1.10x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.23x | 2.93x | 2.20x | 2.87x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.72x | 1.30x | 1.26x | 1.77x | 2.24x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 1.10x | 8.42x | 8.11x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
