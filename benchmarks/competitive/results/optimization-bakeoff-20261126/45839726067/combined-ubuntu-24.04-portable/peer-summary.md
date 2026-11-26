| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.16x | 1.62x | 1.46x | 2.04x | 1.46x | 30/30 |
| materialize | 1.00x | 2.28x | 1.82x | 1.33x | 2.40x | 1.76x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.04x | 1.62x | 1.28x | 2.03x | 1.76x | 30/30 |
| perft_full_d3 | 0.99x | 0.97x | 1.17x | 0.89x | 1.35x | 1.28x | 22/30 |
| fen_parse | 1.02x | 1.81x | 1.67x | 1.31x | 2.09x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 2.67x | 8.87x | 1.72x | 26.25x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 2.84x | 1.13x | 0.97x | 24/30 |
| san_parse_all | 1.01x | 2.19x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.64x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.25x | 1.37x | 5.20x | 1.31x | 1.35x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.20x | 107.81x | 156.79x | 107.98x | 5/5 |
| chess960_materialize | 0.96x | 1.90x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.44x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.20x | 1.77x | 1.24x | 1.93x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 2.07x | 1.84x | 1.35x | 3.04x | 1.71x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.06x | 2.94x | 2.23x | 2.86x | 1.59x | 10/10 |
| replay_trusted_native | 0.89x | 0.85x | 1.68x | 1.87x | 2.73x | 2.48x | 8/10 |
| replay_checked_native | 1.07x | 2.59x | 1.14x | 6.20x | 6.67x | 7.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
