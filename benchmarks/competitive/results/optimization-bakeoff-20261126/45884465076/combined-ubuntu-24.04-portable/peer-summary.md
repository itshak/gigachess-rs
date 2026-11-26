| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.87x | 1.62x | 1.41x | 1.86x | 1.34x | 29/30 |
| materialize | 0.98x | 2.39x | 1.85x | 1.85x | 2.79x | 1.97x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.98x | 1.59x | 1.18x | 1.95x | 1.76x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.58x | 1.39x | 1.83x | 1.57x | 30/30 |
| fen_parse | 1.00x | 1.44x | 1.29x | 1.05x | 1.34x | 1.09x | 30/30 |
| fen_write_native_ep | 1.00x | 2.98x | 9.89x | 1.78x | 27.62x | 2.26x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 29/30 |
| san_parse_all | 1.01x | 1.92x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.44x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.24x | 1.45x | 4.24x | 1.38x | 1.58x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 62.91x | 104.97x | 121.12x | 99.20x | 5/5 |
| chess960_materialize | 1.01x | 1.85x | 1.83x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.14x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 41.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 3.98x | 2.01x | 1.09x | 2.03x | 2.72x | 10/10 |
| corpus_materialize | 1.00x | 1.48x | 1.41x | 1.13x | 1.84x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.93x | 2.95x | 2.20x | 2.89x | 1.52x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.71x | 1.66x | 2.26x | 2.08x | 9/10 |
| replay_checked_native | 1.01x | 2.09x | 1.20x | 8.93x | 8.69x | 10.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
