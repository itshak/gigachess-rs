| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.68x | 1.42x | 2.17x | 1.74x | 30/30 |
| materialize | 0.94x | 1.95x | 1.48x | 1.63x | 2.32x | 1.73x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.55x | 1.18x | 2.32x | 1.87x | 30/30 |
| perft_full_d3 | 0.98x | 0.88x | 1.00x | 0.97x | 1.31x | 1.06x | 16/30 |
| fen_parse | 0.95x | 1.37x | 1.29x | 1.13x | 1.42x | 1.24x | 30/30 |
| fen_write_native_ep | 0.98x | 2.81x | 6.00x | 1.13x | 17.95x | 1.53x | 30/30 |
| snapshot | 0.77x | 1.41x | 1.15x | 3.22x | 1.15x | 1.16x | 30/30 |
| san_parse_all | 1.00x | 1.80x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 8.91x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.59x | 0.99x | 2.91x | 1.02x | 1.21x | 25/30 |
| make_unmake_full | 1.05x | N/A | N/A | 0.99x | N/A | N/A | 2/6 |
| startpos | 1.10x | 1.08x | 72.28x | 134.10x | 157.54x | 139.88x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.17x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.99x | 21.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.28x | 2.36x | 1.30x | 2.94x | 2.65x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.33x | 1.10x | 1.77x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.38x | 1.82x | 1.39x | 1.81x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.61x | 1.10x | 1.10x | 1.55x | 1.83x | 7/10 |
| replay_checked_native | 1.01x | 1.81x | 1.00x | 7.56x | 7.49x | 8.73x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
