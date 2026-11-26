| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 2.57x | 1.48x | 1.29x | 1.71x | 1.23x | 28/30 |
| materialize | 1.17x | 2.18x | 1.75x | 1.77x | 2.67x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.20x | 2.92x | 1.55x | 1.15x | 1.91x | 1.72x | 30/30 |
| perft_full_d3 | 1.52x | 0.99x | 1.13x | 1.01x | 1.37x | 1.14x | 22/30 |
| fen_parse | 1.19x | 1.75x | 1.49x | 1.24x | 1.63x | 1.22x | 30/30 |
| fen_write_native_ep | 0.95x | 1.82x | 6.18x | 1.22x | 17.20x | 1.44x | 30/30 |
| snapshot | 1.79x | 1.48x | 1.12x | 3.02x | 1.12x | 1.00x | 28/30 |
| san_parse_all | 2.15x | 1.67x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 2.54x | 9.04x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 2.32x | 1.58x | 1.05x | 3.02x | 0.99x | 1.14x | 25/30 |
| make_unmake_full | 1.29x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 1.71x | 1.35x | 57.06x | 105.65x | 130.37x | 105.00x | 5/5 |
| chess960_materialize | 1.17x | 1.58x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.28x | 1.80x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 39.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.49x | 22.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.51x | 2.26x | 1.24x | 2.31x | 3.08x | 10/10 |
| corpus_materialize | 1.02x | 1.54x | 1.43x | 1.17x | 1.86x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.52x | 1.21x | 1.84x | 1.36x | 1.78x | 0.97x | 8/10 |
| replay_trusted_native | 1.23x | 0.63x | 1.13x | 1.13x | 1.56x | 1.43x | 7/10 |
| replay_checked_native | 1.22x | 1.72x | 0.97x | 7.12x | 7.04x | 7.99x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
