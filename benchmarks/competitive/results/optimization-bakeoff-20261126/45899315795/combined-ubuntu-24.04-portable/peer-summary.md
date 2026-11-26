| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.99x | 1.56x | 1.32x | 1.88x | 1.33x | 30/30 |
| materialize | 0.96x | 2.21x | 1.71x | 1.28x | 2.53x | 1.58x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.99x | 1.53x | 1.16x | 1.88x | 1.75x | 30/30 |
| perft_full_d3 | 0.98x | 1.38x | 1.69x | 1.29x | 1.91x | 1.73x | 30/30 |
| fen_parse | 1.11x | 1.56x | 1.54x | 1.24x | 1.99x | 1.26x | 30/30 |
| fen_write_native_ep | 0.92x | 2.46x | 7.76x | 1.36x | 23.03x | 1.76x | 30/30 |
| snapshot | 1.03x | 1.64x | 1.15x | 2.63x | 1.15x | 1.00x | 27/30 |
| san_parse_all | 1.01x | 1.77x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 9.76x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.17x | 1.46x | 5.75x | 1.28x | 1.32x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.15x | 1.50x | 69.45x | 95.90x | 152.87x | 95.76x | 5/5 |
| chess960_materialize | 0.91x | 1.67x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.14x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 32.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 22.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.08x | 1.69x | 1.02x | 1.87x | 2.67x | 9/10 |
| corpus_materialize | 1.00x | 1.75x | 1.57x | 1.18x | 2.27x | 1.42x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.02x | 2.85x | 2.22x | 2.92x | 1.61x | 10/10 |
| replay_trusted_native | 0.95x | 0.70x | 1.63x | 1.58x | 2.24x | 1.93x | 8/10 |
| replay_checked_native | 0.94x | 2.11x | 1.20x | 8.42x | 8.06x | 9.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
