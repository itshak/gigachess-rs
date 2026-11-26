| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.23x | 1.82x | 1.54x | 2.39x | 1.88x | 30/30 |
| materialize | 0.94x | 2.08x | 1.61x | 1.69x | 2.60x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.61x | 1.49x | 1.13x | 2.33x | 1.79x | 29/30 |
| perft_full_d3 | 0.96x | 1.30x | 1.47x | 1.45x | 2.18x | 1.60x | 30/30 |
| fen_parse | 0.92x | 1.15x | 1.29x | 1.08x | 1.46x | 1.04x | 25/30 |
| fen_write_native_ep | 1.00x | 4.54x | 10.46x | 2.05x | 30.86x | 2.19x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.05x | 1.85x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.52x | N/A | 1.99x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.30x | 3.95x | 1.30x | 1.64x | 30/30 |
| make_unmake_full | 0.87x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 0.83x | 0.84x | 65.73x | 122.35x | 157.38x | 120.69x | 4/5 |
| chess960_materialize | 0.95x | 1.64x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.31x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.80x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 4.54x | 1.72x | 0.95x | 2.13x | 1.90x | 8/10 |
| corpus_materialize | 0.99x | 1.38x | 1.29x | 1.06x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.19x | 2.99x | 2.19x | 2.88x | 1.53x | 10/10 |
| replay_trusted_native | 0.94x | 0.81x | 1.43x | 1.49x | 2.07x | 2.87x | 9/10 |
| replay_checked_native | 0.94x | 1.88x | 0.99x | 8.08x | 7.77x | 9.22x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
