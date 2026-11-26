| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.62x | 1.54x | 1.57x | 2.42x | 1.79x | 30/30 |
| materialize | 0.94x | 2.10x | 1.61x | 1.19x | 2.14x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.14x | 1.60x | 1.33x | 2.53x | 2.11x | 30/30 |
| perft_full_d3 | 1.05x | 0.86x | 0.89x | 0.67x | 1.24x | 1.01x | 9/30 |
| fen_parse | 1.03x | 1.29x | 1.49x | 1.09x | 1.83x | 1.31x | 30/30 |
| fen_write_native_ep | 1.54x | 4.04x | 10.03x | 1.86x | 27.41x | 2.09x | 30/30 |
| snapshot | 0.64x | 1.52x | 1.15x | 2.41x | 2.19x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 2.15x | N/A | 3.61x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.22x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.31x | 0.88x | 3.17x | 1.00x | 1.16x | 22/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.97x | N/A | N/A | 3/6 |
| startpos | 1.20x | 1.40x | 93.66x | 139.38x | 219.89x | 164.08x | 5/5 |
| chess960_materialize | 0.99x | 1.66x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.78x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.77x | 1.82x | 1.42x | 2.55x | 2.10x | 10/10 |
| corpus_materialize | 1.00x | 1.97x | 1.77x | 1.28x | 2.87x | 1.58x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.35x | 1.83x | 1.36x | 1.71x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.65x | 1.04x | 1.22x | 2.17x | 2.68x | 7/10 |
| replay_checked_native | 1.01x | 2.62x | 0.94x | 5.45x | 6.08x | 7.27x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
