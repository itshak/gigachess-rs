| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.68x | 1.83x | 2.17x | 2.16x | 30/30 |
| materialize | 1.01x | 1.96x | 1.52x | 1.34x | 2.17x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.57x | 1.43x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.61x | 1.33x | 1.97x | 1.87x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.26x | 1.75x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.43x | 1.56x | 30.01x | 2.40x | 30/30 |
| snapshot | 0.98x | 1.05x | 1.06x | 1.45x | 1.08x | 0.96x | 22/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.42x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.25x | 1.48x | 4.00x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.38x | 128.74x | 171.00x | 136.44x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.94x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 26.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.46x | 1.70x | 1.36x | 2.00x | 2.22x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.52x | 1.22x | 1.89x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.91x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.95x | 0.60x | 1.59x | 1.91x | 2.15x | 2.21x | 8/10 |
| replay_checked_native | 1.01x | 2.34x | 1.14x | 7.66x | 6.86x | 8.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
