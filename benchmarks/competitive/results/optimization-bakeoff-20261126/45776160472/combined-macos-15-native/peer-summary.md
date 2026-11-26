| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.22x | 1.32x | 1.36x | 1.77x | 1.59x | 29/30 |
| materialize | 1.01x | 1.93x | 1.41x | 1.03x | 2.62x | 1.26x | 28/30 |
| perft_bulk_d4 | 1.03x | 4.32x | 1.55x | 1.29x | 1.95x | 1.89x | 30/30 |
| perft_full_d3 | 0.98x | 0.95x | 1.00x | 0.79x | 1.16x | 1.17x | 17/30 |
| fen_parse | 0.92x | 1.24x | 1.32x | 1.30x | 1.62x | 1.37x | 30/30 |
| fen_write_native_ep | 1.34x | 3.98x | 8.60x | 1.66x | 26.53x | 2.15x | 30/30 |
| snapshot | 1.02x | 1.70x | 1.09x | 2.40x | 1.09x | 1.05x | 25/30 |
| san_parse_all | 1.02x | 1.72x | N/A | 3.61x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.42x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 0.81x | 0.85x | 4.23x | 0.86x | 0.97x | 9/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 1.03x | 1.27x | 72.87x | 136.51x | 166.10x | 133.27x | 5/5 |
| chess960_materialize | 1.04x | 1.42x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.14x | 3.32x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 37.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 18.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.45x | 2.06x | 1.32x | 2.52x | 2.74x | 10/10 |
| corpus_materialize | 0.99x | 2.04x | 1.43x | 1.29x | 1.97x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.38x | 1.75x | 1.29x | 1.61x | 0.92x | 8/10 |
| replay_trusted_native | 0.95x | 0.44x | 1.12x | 1.13x | 1.29x | 1.21x | 8/10 |
| replay_checked_native | 1.01x | 2.00x | 0.91x | 6.92x | 5.98x | 7.41x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
