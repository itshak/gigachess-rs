| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.99x | 1.73x | 1.93x | 2.45x | 2.20x | 30/30 |
| materialize | 0.98x | 1.88x | 1.47x | 1.22x | 2.33x | 1.40x | 29/30 |
| perft_bulk_d4 | 1.08x | 4.39x | 1.70x | 1.34x | 2.11x | 2.02x | 30/30 |
| perft_full_d3 | 1.11x | 1.69x | 1.90x | 1.50x | 2.20x | 2.26x | 30/30 |
| fen_parse | 0.96x | 1.28x | 1.31x | 1.32x | 1.58x | 1.51x | 30/30 |
| fen_write_native_ep | 0.99x | 4.06x | 8.58x | 1.88x | 28.53x | 2.22x | 30/30 |
| snapshot | 0.95x | 1.67x | 1.14x | 2.44x | 1.19x | 1.02x | 23/30 |
| san_parse_all | 0.99x | 1.78x | N/A | 3.78x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 12.26x | N/A | 2.73x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.22x | 1.32x | 6.14x | 1.19x | 1.38x | 29/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 0.74x | 1.08x | 61.27x | 118.61x | 133.41x | 108.33x | 5/5 |
| chess960_materialize | 0.86x | 1.70x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 3.27x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 6.56x | 2.26x | 1.41x | 2.72x | 2.84x | 10/10 |
| corpus_materialize | 1.07x | 1.62x | 1.47x | 1.20x | 2.00x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.94x | 1.73x | 2.41x | 1.81x | 2.40x | 1.35x | 10/10 |
| replay_trusted_native | 1.11x | 0.73x | 1.72x | 1.88x | 1.74x | 1.80x | 8/10 |
| replay_checked_native | 1.00x | 3.08x | 1.25x | 9.75x | 8.67x | 10.63x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
