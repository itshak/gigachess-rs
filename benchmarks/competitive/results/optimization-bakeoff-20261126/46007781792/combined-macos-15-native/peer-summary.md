| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.46x | 1.92x | 1.87x | 2.25x | 2.29x | 30/30 |
| materialize | 0.85x | 1.70x | 1.32x | 1.04x | 2.05x | 1.35x | 26/30 |
| perft_bulk_d4 | 1.03x | 4.22x | 1.60x | 1.31x | 2.09x | 2.03x | 30/30 |
| perft_full_d3 | 0.97x | 1.85x | 1.87x | 1.31x | 2.04x | 1.99x | 29/30 |
| fen_parse | 0.87x | 1.18x | 1.31x | 1.37x | 1.68x | 1.43x | 30/30 |
| fen_write_native_ep | 0.98x | 4.08x | 8.51x | 1.59x | 26.57x | 2.06x | 30/30 |
| snapshot | 0.87x | 1.74x | 1.19x | 2.28x | 0.96x | 0.90x | 20/30 |
| san_parse_all | 0.95x | 1.80x | N/A | 3.76x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 12.74x | N/A | 2.95x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.15x | 1.19x | 5.61x | 1.14x | 1.19x | 25/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.47x | 1.34x | 90.24x | 191.87x | 208.27x | 189.20x | 5/5 |
| chess960_materialize | 0.83x | 1.50x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.27x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 38.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.85x | 18.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 5.70x | 2.03x | 1.38x | 2.77x | 2.71x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.45x | 1.06x | 1.72x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.96x | 2.70x | 1.80x | 2.33x | 1.32x | 10/10 |
| replay_trusted_native | 1.08x | 0.68x | 1.85x | 1.85x | 1.90x | 2.04x | 8/10 |
| replay_checked_native | 0.99x | 2.51x | 1.28x | 9.53x | 9.60x | 10.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
