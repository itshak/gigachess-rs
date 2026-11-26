| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.78x | 1.48x | 1.25x | 1.77x | 1.24x | 28/30 |
| materialize | 0.99x | 2.08x | 1.62x | 1.21x | 2.41x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.82x | 1.48x | 1.10x | 1.80x | 1.62x | 29/30 |
| perft_full_d3 | 1.01x | 0.81x | 0.98x | 0.76x | 1.10x | 1.01x | 12/30 |
| fen_parse | 0.98x | 1.64x | 1.57x | 1.21x | 1.94x | 1.28x | 30/30 |
| fen_write_native_ep | 0.99x | 2.14x | 6.75x | 1.22x | 20.02x | 1.50x | 30/30 |
| snapshot | 1.01x | 1.64x | 1.16x | 2.67x | 1.16x | 1.00x | 27/30 |
| san_parse_all | 1.07x | 1.74x | N/A | 3.76x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.86x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.43x | 0.94x | 3.55x | 0.82x | 0.88x | 13/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 1.02x | 1.28x | 59.79x | 85.41x | 117.76x | 84.53x | 5/5 |
| chess960_materialize | 0.87x | 1.34x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.92x | 1.25x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 31.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 21.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.26x | 1.76x | 1.07x | 1.91x | 2.74x | 9/10 |
| corpus_materialize | 1.03x | 1.86x | 1.68x | 1.25x | 2.43x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.25x | 1.82x | 1.41x | 1.82x | 1.01x | 9/10 |
| replay_trusted_native | 0.98x | 0.48x | 1.08x | 1.11x | 1.49x | 1.36x | 7/10 |
| replay_checked_native | 1.11x | 1.87x | 1.05x | 7.49x | 7.17x | 8.67x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
