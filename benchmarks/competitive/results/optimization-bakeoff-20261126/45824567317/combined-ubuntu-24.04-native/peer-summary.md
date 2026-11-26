| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.77x | 1.62x | 1.37x | 2.27x | 1.68x | 30/30 |
| materialize | 1.04x | 2.12x | 1.61x | 1.62x | 2.66x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.26x | 1.34x | 1.02x | 2.15x | 1.62x | 28/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.63x | 1.60x | 2.16x | 1.79x | 30/30 |
| fen_parse | 1.07x | 1.44x | 1.39x | 1.14x | 1.59x | 1.26x | 30/30 |
| fen_write_native_ep | 1.01x | 4.71x | 10.44x | 1.93x | 30.77x | 2.60x | 30/30 |
| snapshot | 0.99x | 1.65x | 1.32x | 3.98x | 1.32x | 1.33x | 30/30 |
| san_parse_all | 0.87x | 1.69x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.96x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.06x | 1.33x | 3.96x | 1.28x | 1.68x | 30/30 |
| make_unmake_full | 1.08x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.89x | 0.99x | 78.88x | 148.41x | 183.45x | 155.58x | 4/5 |
| chess960_materialize | 0.96x | 1.66x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.31x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 16.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.17x | 1.60x | 0.87x | 1.93x | 1.77x | 8/10 |
| corpus_materialize | 1.01x | 1.36x | 1.28x | 1.04x | 1.65x | 1.14x | 9/10 |
| corpus_uci_sorted | 0.99x | 2.25x | 3.03x | 2.24x | 2.91x | 1.55x | 10/10 |
| replay_trusted_native | 1.02x | 0.92x | 1.57x | 1.61x | 2.26x | 2.64x | 9/10 |
| replay_checked_native | 1.12x | 2.21x | 1.14x | 9.31x | 8.90x | 10.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
