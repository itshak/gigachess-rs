| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 2.71x | 1.55x | 1.34x | 1.82x | 1.28x | 29/30 |
| materialize | 0.94x | 2.27x | 1.78x | 1.83x | 2.73x | 1.97x | 30/30 |
| perft_bulk_d4 | 0.87x | 2.61x | 1.39x | 1.03x | 1.74x | 1.55x | 28/30 |
| perft_full_d3 | 1.05x | 1.44x | 1.69x | 1.49x | 2.06x | 1.66x | 30/30 |
| fen_parse | 0.97x | 1.53x | 1.30x | 1.10x | 1.34x | 1.11x | 27/30 |
| fen_write_native_ep | 0.91x | 2.80x | 9.38x | 1.71x | 25.50x | 2.08x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 0.95x | 1.82x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.70x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.96x | 1.36x | 3.88x | 1.30x | 1.46x | 30/30 |
| make_unmake_full | 0.78x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 1.40x | 2.00x | 79.80x | 146.83x | 172.57x | 161.47x | 5/5 |
| chess960_materialize | 0.93x | 1.69x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 1.76x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 38.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.08x | 21.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.99x | 1.99x | 1.10x | 2.14x | 2.74x | 10/10 |
| corpus_materialize | 0.98x | 1.47x | 1.38x | 1.10x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.95x | 3.11x | 2.20x | 3.04x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.84x | 1.63x | 1.59x | 2.16x | 2.00x | 8/10 |
| replay_checked_native | 0.92x | 1.96x | 1.07x | 8.17x | 8.03x | 9.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
