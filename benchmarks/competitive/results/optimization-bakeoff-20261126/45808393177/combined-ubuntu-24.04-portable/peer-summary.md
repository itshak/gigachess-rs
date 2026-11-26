| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 2.70x | 1.56x | 1.35x | 1.79x | 1.32x | 29/30 |
| materialize | 0.99x | 2.17x | 1.89x | 1.77x | 2.79x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.91x | 2.70x | 1.43x | 1.06x | 1.76x | 1.61x | 28/30 |
| perft_full_d3 | 0.95x | 1.50x | 1.71x | 1.54x | 2.00x | 1.75x | 30/30 |
| fen_parse | 1.01x | 1.54x | 1.47x | 1.20x | 1.53x | 1.25x | 30/30 |
| fen_write_native_ep | 1.01x | 3.13x | 10.07x | 1.83x | 27.74x | 2.59x | 30/30 |
| snapshot | 1.24x | 1.74x | 1.25x | 3.25x | 1.25x | 1.00x | 29/30 |
| san_parse_all | 0.99x | 1.90x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.45x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.08x | 1.33x | 3.88x | 1.28x | 1.46x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.76x | 108.39x | 125.64x | 103.64x | 5/5 |
| chess960_materialize | 1.05x | 1.62x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 1.97x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 40.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.79x | 16.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.79x | 1.92x | 1.04x | 1.90x | 2.64x | 10/10 |
| corpus_materialize | 0.93x | 1.44x | 1.35x | 1.09x | 1.75x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.90x | 2.87x | 2.17x | 2.81x | 1.51x | 10/10 |
| replay_trusted_native | 0.97x | 0.81x | 1.47x | 1.43x | 1.98x | 1.82x | 8/10 |
| replay_checked_native | 1.00x | 2.01x | 1.10x | 8.58x | 8.51x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
