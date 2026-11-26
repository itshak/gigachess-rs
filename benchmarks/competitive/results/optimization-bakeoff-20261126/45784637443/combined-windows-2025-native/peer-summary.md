| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.77x | 1.59x | 1.50x | 2.22x | 1.96x | 30/30 |
| materialize | 1.04x | 2.06x | 1.35x | 1.18x | 2.31x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.11x | 4.10x | 1.65x | 1.29x | 2.40x | 2.24x | 30/30 |
| perft_full_d3 | 1.09x | 1.17x | 1.54x | 1.04x | 1.77x | 1.99x | 27/30 |
| fen_parse | 1.17x | 1.39x | 1.58x | 1.56x | 2.15x | 1.65x | 30/30 |
| fen_write_native_ep | 1.05x | 3.71x | 7.64x | 1.52x | 31.75x | 2.11x | 30/30 |
| snapshot | 1.21x | 2.33x | 1.39x | 4.18x | 1.40x | 1.39x | 30/30 |
| san_parse_all | 1.04x | 1.98x | N/A | 4.89x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 10.65x | N/A | 3.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.21x | 1.87x | 1.46x | 7.69x | 1.21x | 1.97x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.06x | 1.25x | 90.59x | 174.12x | 234.68x | 177.81x | 5/5 |
| chess960_materialize | 1.18x | 1.57x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 2.77x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 35.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 25.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.18x | 1.83x | 1.21x | 2.41x | 2.28x | 10/10 |
| corpus_materialize | 1.08x | 1.73x | 1.43x | 1.14x | 2.45x | 1.44x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.62x | 1.98x | 1.75x | 1.92x | 1.36x | 10/10 |
| replay_trusted_native | 1.07x | 0.72x | 1.22x | 1.46x | 2.23x | 2.71x | 8/10 |
| replay_checked_native | 1.07x | 2.03x | 0.95x | 6.94x | 7.08x | 8.76x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
