| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.65x | 1.50x | 1.31x | 1.75x | 1.26x | 28/30 |
| materialize | 0.93x | 2.16x | 1.75x | 1.74x | 2.58x | 1.87x | 30/30 |
| perft_bulk_d4 | 0.94x | 2.76x | 1.48x | 1.09x | 1.82x | 1.63x | 30/30 |
| perft_full_d3 | 1.01x | 1.55x | 1.81x | 1.59x | 2.20x | 1.82x | 30/30 |
| fen_parse | 0.90x | 1.35x | 1.30x | 1.09x | 1.36x | 1.10x | 30/30 |
| fen_write_native_ep | 0.97x | 1.96x | 6.44x | 1.16x | 18.16x | 1.60x | 30/30 |
| snapshot | 1.12x | 1.51x | 1.25x | 3.37x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.96x | 1.80x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.88x | N/A | 2.06x | N/A | N/A | 12/12 |
| apply_children_full | 0.70x | 1.24x | 0.83x | 2.36x | 0.75x | 0.88x | 12/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.01x | N/A | N/A | 3/6 |
| startpos | 1.16x | 1.49x | 70.17x | 126.72x | 141.56x | 114.49x | 5/5 |
| chess960_materialize | 0.99x | 1.64x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 1.71x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 37.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 19.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.62x | 2.33x | 1.27x | 2.36x | 3.16x | 10/10 |
| corpus_materialize | 1.00x | 1.54x | 1.45x | 1.17x | 1.88x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.97x | 3.09x | 2.23x | 2.98x | 1.58x | 10/10 |
| replay_trusted_native | 0.97x | 0.66x | 1.29x | 1.27x | 1.71x | 1.58x | 8/10 |
| replay_checked_native | 0.93x | 1.84x | 1.04x | 7.89x | 7.82x | 8.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
