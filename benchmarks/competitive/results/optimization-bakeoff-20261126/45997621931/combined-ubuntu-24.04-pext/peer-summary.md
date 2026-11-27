| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.48x | 1.87x | 1.63x | 2.56x | 1.91x | 30/30 |
| materialize | 1.00x | 2.27x | 1.69x | 1.73x | 2.82x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.85x | 1.53x | 1.20x | 2.39x | 1.87x | 29/30 |
| perft_full_d3 | 1.02x | 1.37x | 1.57x | 1.54x | 2.05x | 1.67x | 30/30 |
| fen_parse | 0.94x | 1.40x | 1.24x | 1.10x | 1.57x | 1.19x | 29/30 |
| fen_write_native_ep | 1.01x | 4.99x | 10.33x | 2.07x | 29.39x | 2.54x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 28/30 |
| san_parse_all | 1.03x | 1.92x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.11x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.05x | 1.44x | 1.66x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.09x | N/A | N/A | 5/6 |
| startpos | 0.89x | 1.00x | 78.23x | 139.45x | 174.45x | 155.51x | 5/5 |
| chess960_materialize | 1.03x | 1.66x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.51x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.11x | 1.91x | 1.06x | 2.35x | 2.14x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.09x | 1.77x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.32x | 3.08x | 2.26x | 3.02x | 1.57x | 10/10 |
| replay_trusted_native | 0.90x | 0.80x | 1.44x | 1.48x | 2.03x | 2.33x | 9/10 |
| replay_checked_native | 0.97x | 2.11x | 1.12x | 8.93x | 8.73x | 10.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
