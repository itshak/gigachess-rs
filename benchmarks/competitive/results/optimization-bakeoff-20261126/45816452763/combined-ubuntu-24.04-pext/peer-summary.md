| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.07x | 1.67x | 1.45x | 2.21x | 1.71x | 30/30 |
| materialize | 1.01x | 2.17x | 1.58x | 1.67x | 2.50x | 1.79x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.47x | 1.37x | 1.07x | 2.08x | 1.75x | 29/30 |
| perft_full_d3 | 0.92x | 1.35x | 1.57x | 1.52x | 1.96x | 1.61x | 30/30 |
| fen_parse | 1.06x | 1.47x | 1.56x | 1.13x | 1.46x | 1.12x | 30/30 |
| fen_write_native_ep | 1.02x | 4.55x | 10.11x | 1.94x | 28.56x | 2.61x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.01x | 1.34x | 1.33x | 30/30 |
| san_parse_all | 1.03x | 1.97x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.15x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.83x | 1.15x | 3.51x | 1.08x | 1.50x | 28/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.01x | N/A | N/A | 3/6 |
| startpos | 1.25x | 1.26x | 99.77x | 179.60x | 221.54x | 182.23x | 5/5 |
| chess960_materialize | 0.87x | 1.43x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.26x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 41.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 17.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.45x | 1.66x | 0.93x | 2.03x | 1.88x | 8/10 |
| corpus_materialize | 1.00x | 1.37x | 1.27x | 1.07x | 1.70x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 2.94x | 2.24x | 2.85x | 1.56x | 10/10 |
| replay_trusted_native | 0.93x | 0.77x | 1.39x | 1.39x | 1.90x | 2.21x | 8/10 |
| replay_checked_native | 0.96x | 1.99x | 1.09x | 8.47x | 8.09x | 9.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
