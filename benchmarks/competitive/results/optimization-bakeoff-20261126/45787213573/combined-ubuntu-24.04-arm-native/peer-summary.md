| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.46x | 1.58x | 1.91x | 1.89x | 30/30 |
| materialize | 1.00x | 1.81x | 1.42x | 1.28x | 1.99x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.53x | 1.39x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.97x | 1.11x | 1.21x | 1.01x | 1.45x | 1.41x | 26/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.30x | 1.78x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.35x | 1.56x | 29.90x | 2.38x | 30/30 |
| snapshot | 1.00x | 1.13x | 0.97x | 1.44x | 0.97x | 0.89x | 12/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.61x | N/A | 2.49x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.11x | 1.31x | 3.63x | 1.07x | 1.06x | 27/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.19x | 70.67x | 122.56x | 160.25x | 128.83x | 5/5 |
| chess960_materialize | 0.95x | 1.44x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.52x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.69x | 1.82x | 1.46x | 2.11x | 2.34x | 10/10 |
| corpus_materialize | 0.99x | 1.57x | 1.50x | 1.22x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.89x | 2.07x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 0.90x | 0.45x | 1.18x | 1.44x | 1.58x | 1.64x | 8/10 |
| replay_checked_native | 0.98x | 2.01x | 0.97x | 6.57x | 5.76x | 7.51x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
