| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.92x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.84x | 1.43x | 1.26x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.48x | 1.41x | 1.29x | 1.80x | 1.86x | 30/30 |
| perft_full_d3 | 1.00x | 1.16x | 1.25x | 1.05x | 1.52x | 1.46x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.33x | 1.28x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.59x | 9.40x | 1.58x | 29.93x | 2.39x | 30/30 |
| snapshot | 1.05x | 1.22x | 1.03x | 1.60x | 1.04x | 1.02x | 28/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.75x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.12x | 1.32x | 3.63x | 1.08x | 1.08x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.17x | 71.57x | 122.06x | 159.62x | 128.78x | 5/5 |
| chess960_materialize | 1.05x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.70x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 18.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.93x | 1.50x | 1.22x | 1.75x | 1.94x | 10/10 |
| corpus_materialize | 0.97x | 1.54x | 1.45x | 1.17x | 1.81x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.09x | 2.89x | 2.08x | 2.79x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.50x | 1.33x | 1.60x | 1.78x | 1.85x | 8/10 |
| replay_checked_native | 0.99x | 2.01x | 0.98x | 6.56x | 5.87x | 7.59x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
