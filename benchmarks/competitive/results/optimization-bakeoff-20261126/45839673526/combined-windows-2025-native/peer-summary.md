| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.69x | 1.61x | 1.57x | 2.28x | 2.02x | 30/30 |
| materialize | 0.99x | 2.01x | 1.37x | 1.25x | 2.28x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.89x | 1.52x | 1.33x | 2.17x | 1.95x | 30/30 |
| perft_full_d3 | 0.88x | 1.01x | 1.05x | 1.00x | 1.38x | 1.34x | 23/30 |
| fen_parse | 1.14x | 1.36x | 1.52x | 1.31x | 2.19x | 1.52x | 30/30 |
| fen_write_native_ep | 1.00x | 4.20x | 8.13x | 1.56x | 30.75x | 1.99x | 30/30 |
| snapshot | 0.67x | 2.83x | 2.61x | 2.18x | 2.61x | 1.00x | 27/30 |
| san_parse_all | 0.90x | 1.87x | N/A | 5.01x | N/A | N/A | 12/12 |
| san_render_native | 0.85x | 9.39x | N/A | 3.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.11x | 1.31x | 8.82x | 1.36x | 1.47x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 0.86x | 2.28x | 70.58x | 132.45x | 202.37x | 152.56x | 5/5 |
| chess960_materialize | 0.98x | 1.48x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 2.59x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 27.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 21.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.42x | 1.78x | 1.37x | 2.41x | 2.43x | 10/10 |
| corpus_materialize | 1.03x | 1.61x | 1.28x | 1.18x | 2.05x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.78x | 2.19x | 1.90x | 2.15x | 1.42x | 10/10 |
| replay_trusted_native | 0.95x | 0.77x | 1.53x | 1.95x | 2.52x | 2.95x | 8/10 |
| replay_checked_native | 0.90x | 2.05x | 0.91x | 7.01x | 6.94x | 8.28x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
