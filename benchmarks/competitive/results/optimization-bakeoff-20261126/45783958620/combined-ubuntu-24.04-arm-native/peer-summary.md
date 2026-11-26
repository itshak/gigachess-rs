| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.82x | 1.46x | 1.57x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.81x | 1.42x | 1.25x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.52x | 1.37x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.96x | 1.12x | 1.22x | 1.02x | 1.47x | 1.41x | 27/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.30x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.48x | 9.33x | 1.57x | 29.35x | 2.39x | 30/30 |
| snapshot | 0.95x | 1.17x | 0.97x | 1.53x | 0.96x | 0.95x | 12/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.53x | N/A | 2.48x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.13x | 1.32x | 3.63x | 1.09x | 1.09x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.19x | 73.35x | 124.91x | 166.37x | 134.04x | 5/5 |
| chess960_materialize | 0.95x | 1.44x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.49x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.06x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.72x | 1.82x | 1.46x | 2.15x | 2.38x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.51x | 1.22x | 1.88x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.92x | 2.09x | 2.83x | 1.46x | 10/10 |
| replay_trusted_native | 0.91x | 0.46x | 1.21x | 1.45x | 1.61x | 1.66x | 8/10 |
| replay_checked_native | 0.98x | 2.00x | 0.97x | 6.58x | 5.83x | 7.49x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
