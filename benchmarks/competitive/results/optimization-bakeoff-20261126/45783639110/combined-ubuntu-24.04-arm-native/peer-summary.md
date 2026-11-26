| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.86x | 1.47x | 1.58x | 1.91x | 1.90x | 30/30 |
| materialize | 1.00x | 1.81x | 1.49x | 1.25x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.52x | 1.38x | 1.96x | 2.00x | 30/30 |
| perft_full_d3 | 1.00x | 1.17x | 1.27x | 1.08x | 1.55x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.35x | 1.27x | 1.76x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 2.99x | 6.17x | 1.04x | 19.77x | 1.58x | 29/30 |
| snapshot | 0.99x | 1.22x | 0.94x | 1.57x | 0.86x | 1.02x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.80x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.11x | 1.27x | 3.58x | 1.06x | 1.09x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.18x | 77.64x | 130.10x | 173.67x | 139.07x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.64x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.77x | 1.84x | 1.45x | 2.15x | 2.38x | 10/10 |
| corpus_materialize | 1.01x | 1.57x | 1.50x | 1.19x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.09x | 2.92x | 2.09x | 2.78x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.51x | 1.35x | 1.61x | 1.76x | 1.82x | 8/10 |
| replay_checked_native | 0.99x | 2.02x | 0.98x | 6.62x | 5.88x | 7.65x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
