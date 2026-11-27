| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.73x | 1.66x | 1.60x | 2.34x | 2.04x | 30/30 |
| materialize | 0.99x | 2.04x | 1.39x | 1.30x | 2.41x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.20x | 1.71x | 1.47x | 2.44x | 2.16x | 30/30 |
| perft_full_d3 | 1.21x | 1.72x | 1.90x | 1.66x | 2.48x | 2.31x | 30/30 |
| fen_parse | 0.99x | 1.27x | 1.40x | 1.50x | 2.24x | 1.49x | 29/30 |
| fen_write_native_ep | 0.94x | 4.01x | 8.38x | 1.60x | 30.22x | 1.96x | 30/30 |
| snapshot | 1.00x | 2.05x | 1.05x | 4.30x | 1.01x | 1.07x | 24/30 |
| san_parse_all | 1.00x | 2.07x | N/A | 5.71x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.39x | N/A | 3.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.33x | 1.44x | 9.98x | 1.50x | 1.68x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 82.82x | 193.86x | 252.58x | 193.23x | 5/5 |
| chess960_materialize | 0.97x | 1.66x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.90x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 26.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.54x | 1.97x | 1.52x | 2.57x | 2.63x | 10/10 |
| corpus_materialize | 0.98x | 1.62x | 1.26x | 1.13x | 2.04x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.77x | 2.22x | 1.89x | 2.22x | 1.40x | 10/10 |
| replay_trusted_native | 0.95x | 0.87x | 1.88x | 2.14x | 2.78x | 2.98x | 9/10 |
| replay_checked_native | 0.99x | 2.60x | 1.09x | 8.18x | 8.57x | 10.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
