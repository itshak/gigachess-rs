| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.93x | 1.69x | 1.43x | 2.22x | 1.75x | 30/30 |
| materialize | 1.05x | 2.06x | 1.64x | 1.61x | 2.58x | 1.85x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.73x | 1.54x | 1.17x | 2.33x | 1.86x | 30/30 |
| perft_full_d3 | 0.99x | 0.88x | 1.01x | 0.97x | 1.30x | 1.06x | 16/30 |
| fen_parse | 1.03x | 1.38x | 1.56x | 1.38x | 1.43x | 1.25x | 30/30 |
| fen_write_native_ep | 1.04x | 3.28x | 6.51x | 1.23x | 17.89x | 1.61x | 30/30 |
| snapshot | 1.82x | 1.83x | 1.16x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.07x | 1.81x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 8.97x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.57x | 1.00x | 2.82x | 1.00x | 1.18x | 25/30 |
| make_unmake_full | 0.99x | N/A | N/A | 0.94x | N/A | N/A | 1/6 |
| startpos | 1.34x | 1.13x | 88.66x | 195.86x | 191.04x | 176.78x | 5/5 |
| chess960_materialize | 0.93x | 1.50x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.19x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 19.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.34x | 2.39x | 1.31x | 2.88x | 2.69x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.34x | 1.11x | 1.78x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.44x | 1.86x | 1.41x | 1.81x | 0.97x | 8/10 |
| replay_trusted_native | 0.99x | 0.61x | 1.10x | 1.12x | 1.51x | 1.77x | 7/10 |
| replay_checked_native | 0.93x | 1.65x | 0.92x | 6.93x | 6.63x | 7.97x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
