| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 5.19x | 1.83x | 1.74x | 2.51x | 2.19x | 30/30 |
| materialize | 1.00x | 1.99x | 1.34x | 1.32x | 2.30x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.47x | 1.74x | 1.46x | 2.44x | 2.25x | 30/30 |
| perft_full_d3 | 1.15x | 1.63x | 1.77x | 1.56x | 2.33x | 2.23x | 30/30 |
| fen_parse | 1.05x | 1.38x | 1.45x | 1.34x | 2.31x | 1.54x | 30/30 |
| fen_write_native_ep | 0.96x | 4.14x | 7.57x | 1.56x | 32.66x | 1.95x | 30/30 |
| snapshot | 1.03x | 1.97x | 1.00x | 4.06x | 1.05x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 2.09x | N/A | 5.65x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.92x | N/A | 4.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.28x | 1.38x | 9.87x | 1.50x | 1.66x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 82.56x | 147.69x | 230.61x | 173.94x | 5/5 |
| chess960_materialize | 1.02x | 1.72x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.65x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.05x | 24.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 5.94x | 1.92x | 1.46x | 2.61x | 2.57x | 10/10 |
| corpus_materialize | 1.02x | 1.71x | 1.29x | 1.28x | 2.17x | 1.46x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.83x | 2.36x | 1.84x | 2.18x | 1.43x | 10/10 |
| replay_trusted_native | 0.98x | 0.82x | 1.62x | 2.32x | 2.53x | 3.18x | 9/10 |
| replay_checked_native | 0.95x | 2.47x | 1.09x | 8.57x | 7.76x | 9.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
