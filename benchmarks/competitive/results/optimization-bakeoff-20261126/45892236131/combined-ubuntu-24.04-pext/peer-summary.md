| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.72x | 1.58x | 1.61x | 2.40x | 1.91x | 30/30 |
| materialize | 0.97x | 1.96x | 1.63x | 1.24x | 2.25x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.78x | 3.23x | 1.24x | 1.09x | 1.91x | 1.53x | 28/30 |
| perft_full_d3 | 0.84x | 1.16x | 1.26x | 1.12x | 1.65x | 1.73x | 30/30 |
| fen_parse | 0.96x | 1.37x | 1.50x | 1.24x | 2.15x | 1.30x | 30/30 |
| fen_write_native_ep | 0.94x | 3.87x | 9.25x | 1.62x | 27.20x | 1.93x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.25x | 3.27x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.88x | 1.90x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.32x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 2.02x | 1.17x | 3.90x | 1.27x | 1.50x | 30/30 |
| make_unmake_full | 0.78x | N/A | N/A | 1.00x | N/A | N/A | 1/6 |
| startpos | 1.50x | 1.88x | 119.03x | 229.65x | 343.23x | 226.80x | 5/5 |
| chess960_materialize | 0.95x | 1.59x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.41x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 31.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 22.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.43x | 1.71x | 1.37x | 2.55x | 2.26x | 10/10 |
| corpus_materialize | 0.98x | 1.64x | 1.57x | 1.17x | 2.15x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.60x | 3.42x | 2.56x | 3.32x | 1.79x | 10/10 |
| replay_trusted_native | 0.75x | 0.69x | 1.20x | 1.48x | 2.00x | 2.30x | 8/10 |
| replay_checked_native | 0.81x | 2.09x | 0.91x | 6.90x | 6.88x | 8.13x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
