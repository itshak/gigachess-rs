| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.10x | 1.72x | 1.49x | 2.30x | 1.74x | 30/30 |
| materialize | 0.99x | 2.14x | 1.58x | 1.67x | 2.52x | 1.80x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.30x | 1.32x | 1.04x | 2.05x | 1.61x | 28/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.69x | 1.62x | 2.11x | 1.75x | 30/30 |
| fen_parse | 1.06x | 1.39x | 1.57x | 1.14x | 1.47x | 1.25x | 30/30 |
| fen_write_native_ep | 0.99x | 4.60x | 9.95x | 1.92x | 27.91x | 2.52x | 30/30 |
| snapshot | 0.99x | 1.83x | 1.15x | 4.32x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.01x | 1.96x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.15x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.89x | 1.19x | 3.61x | 1.19x | 1.50x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 88.10x | 166.77x | 196.97x | 175.51x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.41x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 16.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.67x | 1.74x | 0.97x | 2.22x | 1.91x | 8/10 |
| corpus_materialize | 1.00x | 1.36x | 1.25x | 1.05x | 1.70x | 1.13x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.26x | 2.94x | 2.23x | 2.86x | 1.54x | 10/10 |
| replay_trusted_native | 1.00x | 0.84x | 1.49x | 1.51x | 2.04x | 2.37x | 9/10 |
| replay_checked_native | 1.00x | 2.10x | 1.11x | 8.74x | 8.47x | 9.82x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
