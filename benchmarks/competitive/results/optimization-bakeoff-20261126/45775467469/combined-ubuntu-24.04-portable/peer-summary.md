| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.81x | 1.41x | 1.30x | 1.80x | 1.28x | 28/30 |
| materialize | 1.01x | 2.00x | 1.74x | 1.27x | 2.31x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.82x | 1.50x | 1.18x | 1.90x | 1.64x | 30/30 |
| perft_full_d3 | 1.46x | 1.36x | 1.55x | 1.28x | 1.86x | 1.69x | 30/30 |
| fen_parse | 1.07x | 1.67x | 1.53x | 1.31x | 2.28x | 1.36x | 30/30 |
| fen_write_native_ep | 1.00x | 1.59x | 5.43x | 0.95x | 15.30x | 1.24x | 25/30 |
| snapshot | 1.00x | 1.68x | 1.21x | 3.04x | 1.02x | 0.98x | 25/30 |
| san_parse_all | 0.98x | 1.86x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.69x | N/A | 2.36x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.53x | 1.05x | 3.05x | 0.96x | 0.98x | 19/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.03x | 1.28x | 64.17x | 117.70x | 176.13x | 115.62x | 5/5 |
| chess960_materialize | 1.00x | 1.43x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.92x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 22.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 17.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.77x | 1.85x | 1.26x | 2.06x | 2.49x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.63x | 1.21x | 2.08x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.24x | 1.84x | 1.39x | 1.77x | 1.01x | 10/10 |
| replay_trusted_native | 1.01x | 0.60x | 1.19x | 1.35x | 1.75x | 1.63x | 8/10 |
| replay_checked_native | 0.98x | 1.97x | 0.97x | 6.92x | 6.86x | 7.65x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
