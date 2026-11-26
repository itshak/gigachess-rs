| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.49x | 1.60x | 1.93x | 1.94x | 30/30 |
| materialize | 1.00x | 1.84x | 1.45x | 1.27x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.41x | 1.40x | 1.27x | 1.78x | 1.84x | 30/30 |
| perft_full_d3 | 0.89x | 1.04x | 1.13x | 0.94x | 1.39x | 1.31x | 24/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.26x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.44x | 9.45x | 1.61x | 29.77x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.12x | 1.05x | 1.47x | 1.06x | 0.95x | 24/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.30x | N/A | 2.46x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.20x | 1.41x | 3.84x | 1.15x | 1.15x | 28/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 75.99x | 130.30x | 175.43x | 143.33x | 5/5 |
| chess960_materialize | 0.94x | 1.49x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.53x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.02x | 1.52x | 1.24x | 1.76x | 2.08x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.48x | 1.17x | 1.82x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.13x | 2.94x | 2.08x | 2.86x | 1.48x | 10/10 |
| replay_trusted_native | 0.85x | 0.52x | 1.38x | 1.66x | 1.87x | 1.91x | 8/10 |
| replay_checked_native | 0.98x | 2.16x | 1.03x | 6.92x | 6.16x | 8.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
