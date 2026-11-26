| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.47x | 1.58x | 1.90x | 1.91x | 30/30 |
| materialize | 1.00x | 1.82x | 1.43x | 1.27x | 2.01x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.52x | 1.39x | 1.94x | 2.00x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.28x | 1.07x | 1.57x | 1.48x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.28x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.59x | 9.42x | 1.58x | 30.34x | 2.39x | 30/30 |
| snapshot | 1.03x | 1.28x | 0.92x | 1.62x | 0.91x | 1.00x | 17/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.82x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.21x | 1.40x | 3.97x | 1.16x | 1.18x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.18x | 76.09x | 128.23x | 172.38x | 137.95x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.74x | 1.83x | 1.46x | 2.13x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.51x | 1.22x | 1.88x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.78x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.51x | 1.32x | 1.60x | 1.77x | 1.83x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.98x | 6.63x | 5.86x | 7.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
