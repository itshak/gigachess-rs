| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.47x | 1.54x | 1.58x | 2.57x | 1.79x | 30/30 |
| materialize | 1.08x | 2.12x | 1.77x | 1.16x | 2.33x | 1.44x | 30/30 |
| perft_bulk_d4 | 1.30x | 4.34x | 1.61x | 1.35x | 2.58x | 2.11x | 30/30 |
| perft_full_d3 | 1.12x | 0.82x | 0.86x | 0.64x | 1.18x | 0.96x | 8/30 |
| fen_parse | 1.06x | 1.32x | 1.46x | 1.19x | 2.13x | 1.26x | 30/30 |
| fen_write_native_ep | 0.96x | 2.53x | 6.06x | 1.15x | 17.35x | 1.27x | 30/30 |
| snapshot | 1.22x | 1.69x | 1.21x | 3.27x | 1.21x | 1.11x | 30/30 |
| san_parse_all | 1.99x | 2.17x | N/A | 3.58x | N/A | N/A | 12/12 |
| san_render_native | 1.95x | 11.39x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 1.29x | 1.35x | 0.92x | 3.23x | 1.00x | 1.51x | 22/30 |
| make_unmake_full | 1.16x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 1.33x | 1.08x | 83.55x | 125.27x | 212.43x | 136.94x | 5/5 |
| chess960_materialize | 1.05x | 1.73x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.38x | 2.84x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.89x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.44x | 18.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 4.81x | 1.84x | 1.41x | 2.54x | 2.11x | 10/10 |
| corpus_materialize | 1.03x | 1.99x | 1.79x | 1.27x | 2.96x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.50x | 1.34x | 1.83x | 1.37x | 1.72x | 0.97x | 8/10 |
| replay_trusted_native | 1.18x | 0.63x | 1.03x | 1.22x | 2.14x | 2.56x | 7/10 |
| replay_checked_native | 1.27x | 2.44x | 0.89x | 5.23x | 6.24x | 6.94x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
