| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.19x | 1.44x | 1.46x | 2.15x | 1.90x | 28/30 |
| materialize | 1.17x | 2.27x | 1.67x | 1.11x | 2.35x | 1.40x | 28/30 |
| perft_bulk_d4 | 1.05x | 4.29x | 1.68x | 1.33x | 2.03x | 1.85x | 30/30 |
| perft_full_d3 | 0.94x | 1.44x | 1.68x | 1.36x | 1.82x | 2.06x | 30/30 |
| fen_parse | 0.96x | 1.24x | 1.29x | 1.22x | 1.63x | 1.37x | 30/30 |
| fen_write_native_ep | 0.97x | 4.53x | 9.02x | 1.67x | 28.91x | 2.14x | 30/30 |
| snapshot | 0.86x | 1.38x | 0.86x | 1.91x | 0.93x | 0.84x | 17/30 |
| san_parse_all | 1.09x | 1.98x | N/A | 4.73x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.85x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 0.94x | 0.99x | 5.09x | 0.94x | 1.16x | 17/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.20x | 91.26x | 216.88x | 217.70x | 195.86x | 5/5 |
| chess960_materialize | 0.85x | 1.65x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.92x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 34.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 19.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.28x | 7.67x | 2.35x | 1.45x | 2.72x | 3.38x | 10/10 |
| corpus_materialize | 1.13x | 1.65x | 1.61x | 1.21x | 2.13x | 1.32x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.08x | 2.63x | 1.88x | 2.52x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.57x | 1.67x | 1.50x | 1.77x | 1.50x | 7/10 |
| replay_checked_native | 0.91x | 1.89x | 0.83x | 6.97x | 6.50x | 7.15x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
