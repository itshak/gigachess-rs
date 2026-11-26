| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.43x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.96x | 1.53x | 1.33x | 2.14x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.83x | 1.55x | 1.41x | 1.99x | 2.03x | 30/30 |
| perft_full_d3 | 0.99x | 1.47x | 1.59x | 1.33x | 1.94x | 1.84x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.41x | 9.29x | 1.58x | 30.25x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.08x | 1.05x | 1.47x | 1.04x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.32x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.21x | 1.43x | 3.92x | 1.17x | 1.17x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.66x | 128.30x | 172.34x | 136.95x | 5/5 |
| chess960_materialize | 0.99x | 1.69x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.05x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.45x | 1.70x | 1.37x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.53x | 1.23x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.05x | 0.67x | 1.75x | 2.12x | 2.37x | 2.39x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.12x | 7.55x | 6.80x | 8.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
