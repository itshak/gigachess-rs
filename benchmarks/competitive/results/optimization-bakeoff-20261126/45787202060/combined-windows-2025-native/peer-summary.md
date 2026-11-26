| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.39x | 1.49x | 1.44x | 2.16x | 1.82x | 30/30 |
| materialize | 0.97x | 1.96x | 1.26x | 1.08x | 2.13x | 1.40x | 29/30 |
| perft_bulk_d4 | 0.77x | 2.88x | 1.18x | 1.03x | 1.77x | 1.61x | 27/30 |
| perft_full_d3 | 0.87x | 1.02x | 1.09x | 0.88x | 1.45x | 1.26x | 21/30 |
| fen_parse | 0.85x | 1.09x | 1.14x | 1.08x | 1.69x | 1.17x | 27/30 |
| fen_write_native_ep | 0.98x | 3.94x | 7.53x | 1.60x | 31.40x | 2.02x | 30/30 |
| snapshot | 0.50x | 1.48x | 1.15x | 1.18x | 1.16x | 1.11x | 30/30 |
| san_parse_all | 0.86x | 1.68x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.78x | 7.87x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.29x | 0.89x | 5.50x | 0.99x | 1.11x | 20/30 |
| make_unmake_full | 0.55x | N/A | N/A | 0.69x | N/A | N/A | 0/6 |
| startpos | 1.52x | 4.55x | 125.58x | 210.03x | 315.25x | 235.54x | 5/5 |
| chess960_materialize | 0.94x | 1.48x | 1.15x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.70x | 1.90x | 0.99x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 1.18x | 23.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.12x | 19.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.43x | 1.80x | 1.31x | 2.35x | 2.17x | 10/10 |
| corpus_materialize | 1.00x | 1.76x | 1.36x | 1.17x | 2.75x | 1.49x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.85x | 2.35x | 1.93x | 2.23x | 1.48x | 10/10 |
| replay_trusted_native | 0.83x | 0.65x | 1.08x | 1.34x | 2.18x | 2.84x | 7/10 |
| replay_checked_native | 0.71x | 1.55x | 0.67x | 3.89x | 4.53x | 5.53x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
