| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.82x | 1.53x | 1.33x | 1.79x | 1.27x | 29/30 |
| materialize | 1.00x | 2.10x | 1.77x | 1.73x | 2.54x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.93x | 1.55x | 1.16x | 1.94x | 1.73x | 30/30 |
| perft_full_d3 | 1.01x | 1.53x | 1.78x | 1.57x | 2.14x | 1.78x | 30/30 |
| fen_parse | 0.97x | 1.48x | 1.47x | 1.17x | 1.55x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 2.07x | 6.38x | 1.20x | 17.80x | 1.64x | 30/30 |
| snapshot | 0.80x | 1.22x | 1.00x | 2.60x | 1.00x | 1.00x | 17/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.36x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.09x | 1.99x | 1.28x | 3.76x | 1.25x | 1.42x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 58.79x | 110.79x | 130.79x | 99.23x | 5/5 |
| chess960_materialize | 1.08x | 1.66x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.87x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 37.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.88x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.64x | 2.33x | 1.28x | 2.39x | 3.18x | 10/10 |
| corpus_materialize | 1.00x | 1.53x | 1.42x | 1.16x | 1.87x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.97x | 2.96x | 2.21x | 2.90x | 1.55x | 10/10 |
| replay_trusted_native | 1.05x | 0.72x | 1.37x | 1.41x | 1.89x | 1.72x | 8/10 |
| replay_checked_native | 0.92x | 1.78x | 1.01x | 7.86x | 7.69x | 8.77x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
