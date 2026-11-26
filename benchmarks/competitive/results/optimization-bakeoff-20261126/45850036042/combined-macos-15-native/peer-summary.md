| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 5.04x | 1.70x | 1.75x | 2.18x | 2.22x | 30/30 |
| materialize | 1.04x | 1.87x | 1.54x | 1.06x | 2.10x | 1.29x | 28/30 |
| perft_bulk_d4 | 1.09x | 4.05x | 1.57x | 1.32x | 1.92x | 1.85x | 30/30 |
| perft_full_d3 | 1.09x | 1.46x | 1.74x | 1.34x | 1.92x | 2.15x | 30/30 |
| fen_parse | 1.08x | 1.16x | 1.42x | 1.40x | 1.70x | 1.32x | 27/30 |
| fen_write_native_ep | 1.07x | 4.24x | 9.00x | 1.79x | 27.94x | 2.23x | 30/30 |
| snapshot | 1.16x | 1.44x | 1.14x | 2.45x | 1.07x | 1.02x | 26/30 |
| san_parse_all | 1.03x | 1.81x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 11.31x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.07x | 1.13x | 5.68x | 1.06x | 1.20x | 26/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.23x | N/A | N/A | 5/6 |
| startpos | 0.82x | 0.77x | 58.11x | 95.01x | 127.05x | 105.10x | 4/5 |
| chess960_materialize | 0.93x | 1.57x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 3.21x | 1.68x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.21x | 39.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.16x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 21.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.08x | 2.55x | 1.26x | 2.43x | 2.49x | 9/10 |
| corpus_materialize | 1.06x | 1.51x | 1.46x | 1.24x | 2.16x | 1.57x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.79x | 2.33x | 1.59x | 2.25x | 1.18x | 9/10 |
| replay_trusted_native | 0.97x | 0.73x | 1.76x | 1.92x | 2.27x | 2.13x | 8/10 |
| replay_checked_native | 1.10x | 2.43x | 1.13x | 9.37x | 7.86x | 9.69x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
