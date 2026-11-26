| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.17x | 1.54x | 1.48x | 2.35x | 1.77x | 30/30 |
| materialize | 0.98x | 2.02x | 1.66x | 1.16x | 2.08x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.75x | 1.48x | 1.21x | 2.34x | 2.00x | 30/30 |
| perft_full_d3 | 0.97x | 1.21x | 1.26x | 0.97x | 1.79x | 1.53x | 25/30 |
| fen_parse | 1.00x | 1.50x | 1.59x | 1.27x | 2.04x | 1.26x | 30/30 |
| fen_write_native_ep | 1.01x | 4.14x | 9.68x | 1.90x | 27.85x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.25x | 3.71x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.27x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.69x | N/A | 2.02x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.81x | 1.18x | 4.44x | 1.38x | 1.56x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 100.46x | 143.39x | 208.68x | 139.03x | 5/5 |
| chess960_materialize | 1.02x | 1.84x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.87x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 20.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 15.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.01x | 1.61x | 1.19x | 2.15x | 1.87x | 9/10 |
| corpus_materialize | 1.00x | 1.80x | 1.71x | 1.17x | 2.73x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.32x | 3.13x | 2.32x | 2.91x | 1.65x | 10/10 |
| replay_trusted_native | 1.02x | 1.15x | 1.95x | 2.27x | 3.86x | 4.69x | 10/10 |
| replay_checked_native | 0.99x | 2.66x | 1.01x | 5.83x | 6.57x | 7.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
