| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.81x | 1.43x | 1.28x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.69x | 1.52x | 1.39x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.07x | 1.57x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.30x | 1.60x | 30.05x | 2.38x | 30/30 |
| snapshot | 1.02x | 1.29x | 0.93x | 1.62x | 0.92x | 1.01x | 18/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.84x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.13x | 1.31x | 3.67x | 1.10x | 1.10x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.47x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.18x | 76.42x | 130.58x | 169.58x | 135.90x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.69x | 1.82x | 1.45x | 2.16x | 2.47x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.49x | 1.22x | 1.87x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.94x | 2.09x | 2.79x | 1.48x | 10/10 |
| replay_trusted_native | 0.98x | 0.49x | 1.34x | 1.56x | 1.73x | 1.81x | 8/10 |
| replay_checked_native | 0.97x | 2.00x | 0.96x | 6.54x | 5.78x | 7.68x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
