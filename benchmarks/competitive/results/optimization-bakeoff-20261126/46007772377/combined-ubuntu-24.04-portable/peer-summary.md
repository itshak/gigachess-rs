| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.02x | 1.73x | 1.51x | 1.99x | 1.51x | 30/30 |
| materialize | 0.94x | 2.21x | 1.72x | 1.81x | 2.62x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.08x | 3.21x | 1.74x | 1.29x | 2.12x | 1.95x | 29/30 |
| perft_full_d3 | 1.15x | 1.60x | 1.87x | 1.62x | 2.15x | 1.86x | 30/30 |
| fen_parse | 1.01x | 1.54x | 1.43x | 1.22x | 1.50x | 1.28x | 30/30 |
| fen_write_native_ep | 1.01x | 3.04x | 10.09x | 1.89x | 27.72x | 2.56x | 30/30 |
| snapshot | 1.02x | 1.75x | 1.25x | 3.26x | 1.25x | 1.01x | 29/30 |
| san_parse_all | 1.02x | 1.95x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.62x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.26x | 1.46x | 4.32x | 1.39x | 1.56x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.45x | 62.06x | 110.81x | 121.69x | 112.10x | 5/5 |
| chess960_materialize | 0.96x | 1.85x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.18x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 41.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 25.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 4.91x | 2.48x | 1.35x | 2.54x | 3.44x | 10/10 |
| corpus_materialize | 1.01x | 1.53x | 1.43x | 1.21x | 1.90x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.97x | 3.01x | 2.22x | 2.95x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 0.89x | 1.70x | 1.71x | 2.26x | 2.09x | 9/10 |
| replay_checked_native | 1.02x | 2.07x | 1.17x | 8.92x | 8.77x | 9.95x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
