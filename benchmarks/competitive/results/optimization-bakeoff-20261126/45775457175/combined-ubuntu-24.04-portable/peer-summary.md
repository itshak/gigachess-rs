| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.82x | 1.46x | 1.24x | 1.75x | 1.25x | 28/30 |
| materialize | 0.95x | 2.19x | 1.62x | 1.23x | 2.36x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.90x | 1.49x | 1.13x | 1.83x | 1.68x | 30/30 |
| perft_full_d3 | 0.99x | 0.79x | 1.00x | 0.75x | 1.13x | 1.03x | 14/30 |
| fen_parse | 1.06x | 1.66x | 1.65x | 1.34x | 2.05x | 1.33x | 30/30 |
| fen_write_native_ep | 0.99x | 2.29x | 7.12x | 1.16x | 20.62x | 1.63x | 30/30 |
| snapshot | 0.99x | 1.69x | 1.17x | 2.62x | 1.16x | 1.04x | 26/30 |
| san_parse_all | 1.01x | 1.66x | N/A | 3.43x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.94x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.42x | 0.95x | 3.93x | 0.83x | 0.87x | 13/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.02x | N/A | N/A | 3/6 |
| startpos | 0.99x | 1.28x | 60.42x | 94.25x | 119.85x | 83.66x | 5/5 |
| chess960_materialize | 0.99x | 1.52x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.92x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 32.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.21x | 24.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.23x | 1.77x | 1.07x | 1.86x | 2.82x | 9/10 |
| corpus_materialize | 0.94x | 1.83x | 1.72x | 1.26x | 2.36x | 1.49x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.22x | 1.76x | 1.35x | 1.72x | 0.98x | 8/10 |
| replay_trusted_native | 0.99x | 0.51x | 1.15x | 1.14x | 1.57x | 1.41x | 7/10 |
| replay_checked_native | 0.95x | 1.67x | 0.98x | 6.70x | 6.65x | 7.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
