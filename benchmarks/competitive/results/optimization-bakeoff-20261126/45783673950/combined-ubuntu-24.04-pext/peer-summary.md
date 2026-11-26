| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.29x | 1.79x | 1.56x | 2.41x | 1.82x | 30/30 |
| materialize | 0.94x | 2.07x | 1.53x | 1.69x | 2.46x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.06x | 1.62x | 1.27x | 2.52x | 1.97x | 30/30 |
| perft_full_d3 | 0.72x | 1.07x | 1.25x | 1.19x | 1.61x | 1.29x | 28/30 |
| fen_parse | 1.04x | 1.43x | 1.27x | 1.13x | 1.38x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 2.97x | 6.46x | 1.32x | 19.29x | 1.67x | 30/30 |
| snapshot | 0.93x | 1.69x | 1.07x | 3.99x | 1.07x | 1.23x | 30/30 |
| san_parse_all | 0.95x | 1.80x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.11x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.73x | 1.09x | 3.26x | 1.07x | 1.33x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 87.82x | 160.24x | 186.95x | 170.84x | 5/5 |
| chess960_materialize | 0.98x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.37x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 39.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 21.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.76x | 2.52x | 1.40x | 3.10x | 2.85x | 10/10 |
| corpus_materialize | 1.01x | 1.48x | 1.35x | 1.13x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.26x | 3.01x | 2.24x | 2.96x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.70x | 1.28x | 1.28x | 1.79x | 2.10x | 8/10 |
| replay_checked_native | 1.00x | 2.00x | 1.06x | 8.41x | 8.08x | 9.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
