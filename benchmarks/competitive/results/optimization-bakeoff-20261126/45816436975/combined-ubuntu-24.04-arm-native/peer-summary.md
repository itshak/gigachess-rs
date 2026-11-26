| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.92x | 1.49x | 1.60x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.83x | 1.45x | 1.26x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.40x | 1.40x | 1.26x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 0.98x | 1.15x | 1.25x | 1.04x | 1.54x | 1.45x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.35x | 1.27x | 1.79x | 1.43x | 30/30 |
| fen_write_native_ep | 0.99x | 4.46x | 9.23x | 1.57x | 29.69x | 2.38x | 30/30 |
| snapshot | 0.91x | 1.10x | 1.05x | 1.45x | 1.05x | 0.95x | 24/30 |
| san_parse_all | 1.01x | 1.87x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.28x | 1.51x | 4.14x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.52x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.34x | 127.82x | 177.42x | 143.73x | 5/5 |
| chess960_materialize | 1.05x | 1.61x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.70x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.83x | 1.49x | 1.21x | 1.74x | 1.94x | 10/10 |
| corpus_materialize | 0.98x | 1.51x | 1.45x | 1.16x | 1.80x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.06x | 0.55x | 1.44x | 1.75x | 1.96x | 2.00x | 8/10 |
| replay_checked_native | 1.04x | 2.10x | 1.03x | 6.76x | 6.10x | 7.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
