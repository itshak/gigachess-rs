| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.69x | 1.42x | 2.20x | 1.73x | 30/30 |
| materialize | 1.02x | 1.97x | 1.49x | 1.62x | 2.30x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.30x | 1.33x | 1.02x | 2.04x | 1.62x | 28/30 |
| perft_full_d3 | 1.02x | 1.50x | 1.75x | 1.65x | 2.28x | 1.83x | 30/30 |
| fen_parse | 1.12x | 1.46x | 1.38x | 1.18x | 1.51x | 1.18x | 30/30 |
| fen_write_native_ep | 0.99x | 5.14x | 10.14x | 1.87x | 28.62x | 2.48x | 30/30 |
| snapshot | 0.71x | 1.73x | 1.08x | 3.05x | 1.08x | 1.15x | 30/30 |
| san_parse_all | 0.98x | 1.87x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.17x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.07x | 1.34x | 3.87x | 1.29x | 1.62x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.16x | 72.39x | 135.58x | 160.72x | 130.40x | 5/5 |
| chess960_materialize | 0.96x | 1.66x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.31x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.12x | 21.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.22x | 1.59x | 0.87x | 1.96x | 1.79x | 8/10 |
| corpus_materialize | 1.01x | 1.34x | 1.25x | 1.03x | 1.66x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.00x | 2.33x | 3.11x | 2.24x | 3.02x | 1.57x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.62x | 1.61x | 2.26x | 2.61x | 9/10 |
| replay_checked_native | 1.00x | 1.97x | 1.07x | 8.17x | 7.91x | 9.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
