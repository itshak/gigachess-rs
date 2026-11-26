| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.21x | 1.82x | 1.54x | 2.36x | 1.88x | 30/30 |
| materialize | 1.03x | 2.14x | 1.63x | 1.69x | 2.56x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.95x | 1.62x | 1.24x | 2.43x | 1.96x | 30/30 |
| perft_full_d3 | 0.97x | 1.47x | 1.74x | 1.62x | 2.14x | 1.81x | 30/30 |
| fen_parse | 1.13x | 1.44x | 1.42x | 1.21x | 1.53x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 4.71x | 9.84x | 1.92x | 28.82x | 2.55x | 30/30 |
| snapshot | 0.78x | 1.34x | 1.08x | 2.29x | 1.08x | 1.00x | 29/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.22x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.68x | 1.10x | 3.15x | 1.06x | 1.36x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.82x | 1.25x | 66.25x | 125.98x | 144.39x | 118.73x | 5/5 |
| chess960_materialize | 1.14x | 1.79x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.43x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 40.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 22.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 6.68x | 2.51x | 1.40x | 3.06x | 2.83x | 10/10 |
| corpus_materialize | 1.02x | 1.42x | 1.32x | 1.10x | 1.75x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 2.93x | 2.23x | 2.89x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.32x | 1.27x | 1.78x | 2.08x | 8/10 |
| replay_checked_native | 1.00x | 2.02x | 1.07x | 8.52x | 8.13x | 9.71x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
