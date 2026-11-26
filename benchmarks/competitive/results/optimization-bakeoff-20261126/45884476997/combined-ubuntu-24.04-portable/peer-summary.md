| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.19x | 1.65x | 1.47x | 2.04x | 1.42x | 30/30 |
| materialize | 0.93x | 2.26x | 1.80x | 1.33x | 2.37x | 1.65x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.02x | 1.61x | 1.27x | 2.02x | 1.73x | 30/30 |
| perft_full_d3 | 1.21x | 1.75x | 2.05x | 1.57x | 2.44x | 2.25x | 30/30 |
| fen_parse | 1.18x | 1.72x | 1.62x | 1.26x | 2.02x | 1.49x | 30/30 |
| fen_write_native_ep | 1.00x | 2.61x | 8.89x | 1.77x | 26.02x | 1.94x | 30/30 |
| snapshot | 1.00x | 1.73x | 1.23x | 2.85x | 1.14x | 0.98x | 24/30 |
| san_parse_all | 0.99x | 2.24x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.30x | N/A | 2.24x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.49x | 1.51x | 5.88x | 1.46x | 1.50x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.68x | 103.74x | 154.28x | 119.32x | 5/5 |
| chess960_materialize | 1.08x | 1.98x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.53x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.18x | 1.76x | 1.23x | 1.92x | 1.84x | 10/10 |
| corpus_materialize | 1.00x | 2.05x | 1.83x | 1.35x | 3.04x | 1.71x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.11x | 2.97x | 2.25x | 2.87x | 1.61x | 10/10 |
| replay_trusted_native | 0.80x | 0.86x | 1.71x | 1.86x | 2.75x | 2.54x | 9/10 |
| replay_checked_native | 0.97x | 2.53x | 1.11x | 6.02x | 6.64x | 7.66x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
