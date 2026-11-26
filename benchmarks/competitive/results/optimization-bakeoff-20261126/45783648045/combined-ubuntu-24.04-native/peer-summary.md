| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.04x | 1.72x | 1.46x | 2.29x | 1.76x | 30/30 |
| materialize | 1.00x | 2.11x | 1.59x | 1.64x | 2.47x | 1.90x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.87x | 1.59x | 1.20x | 2.45x | 1.90x | 30/30 |
| perft_full_d3 | 1.01x | 1.51x | 1.73x | 1.68x | 2.22x | 1.85x | 30/30 |
| fen_parse | 1.10x | 1.48x | 1.57x | 1.14x | 1.49x | 1.14x | 30/30 |
| fen_write_native_ep | 0.96x | 3.14x | 6.45x | 1.29x | 17.07x | 1.61x | 30/30 |
| snapshot | 0.71x | 1.58x | 1.25x | 2.83x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 1.02x | 1.97x | N/A | 4.49x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.18x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.69x | 1.11x | 3.41x | 1.11x | 1.35x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.09x | N/A | N/A | 4/6 |
| startpos | 0.82x | 1.30x | 81.29x | 145.09x | 179.44x | 144.91x | 5/5 |
| chess960_materialize | 0.99x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.28x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 38.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.45x | 2.43x | 1.33x | 2.96x | 2.70x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.10x | 1.76x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.04x | 2.23x | 2.94x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.33x | 1.30x | 1.83x | 2.12x | 8/10 |
| replay_checked_native | 1.01x | 2.03x | 1.14x | 8.64x | 8.35x | 9.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
