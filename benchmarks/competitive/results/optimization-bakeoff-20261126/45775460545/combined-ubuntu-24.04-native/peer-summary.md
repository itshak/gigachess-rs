| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.14x | 1.47x | 1.42x | 2.26x | 1.71x | 30/30 |
| materialize | 1.01x | 2.12x | 1.64x | 1.09x | 2.07x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.84x | 1.51x | 1.23x | 2.50x | 2.05x | 30/30 |
| perft_full_d3 | 1.08x | 0.87x | 0.91x | 0.68x | 1.27x | 1.08x | 12/30 |
| fen_parse | 1.02x | 1.24x | 1.41x | 1.06x | 1.73x | 1.23x | 28/30 |
| fen_write_native_ep | 1.03x | 2.73x | 6.46x | 1.22x | 19.85x | 1.39x | 30/30 |
| snapshot | 0.81x | 1.76x | 1.22x | 2.82x | 1.22x | 1.11x | 30/30 |
| san_parse_all | 1.03x | 2.06x | N/A | 3.54x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 10.31x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.37x | 0.91x | 3.27x | 1.01x | 1.20x | 22/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 1.10x | 1.18x | 90.96x | 126.73x | 195.76x | 145.73x | 5/5 |
| chess960_materialize | 1.03x | 1.66x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.72x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.33x | 1.74x | 1.29x | 2.34x | 2.01x | 10/10 |
| corpus_materialize | 0.99x | 1.82x | 1.68x | 1.20x | 2.74x | 1.51x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.35x | 1.82x | 1.36x | 1.70x | 0.97x | 8/10 |
| replay_trusted_native | 1.05x | 0.67x | 1.11x | 1.28x | 2.29x | 2.85x | 8/10 |
| replay_checked_native | 1.04x | 2.51x | 0.94x | 5.44x | 6.17x | 7.64x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
