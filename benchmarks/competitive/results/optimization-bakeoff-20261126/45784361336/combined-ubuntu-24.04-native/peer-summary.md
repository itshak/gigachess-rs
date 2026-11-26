| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.32x | 1.52x | 1.45x | 2.31x | 1.76x | 30/30 |
| materialize | 1.00x | 2.13x | 1.68x | 1.12x | 2.13x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.91x | 1.55x | 1.26x | 2.43x | 2.29x | 30/30 |
| perft_full_d3 | 0.99x | 1.23x | 1.28x | 1.00x | 1.79x | 1.54x | 27/30 |
| fen_parse | 1.11x | 1.47x | 1.68x | 1.25x | 2.22x | 1.46x | 30/30 |
| fen_write_native_ep | 0.99x | 4.13x | 9.92x | 1.85x | 28.48x | 2.11x | 30/30 |
| snapshot | 0.80x | 1.53x | 1.15x | 2.39x | 1.14x | 0.97x | 24/30 |
| san_parse_all | 0.98x | 2.19x | N/A | 3.70x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.66x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.54x | 1.04x | 3.70x | 1.18x | 1.33x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.15x | 84.93x | 118.55x | 189.71x | 134.15x | 5/5 |
| chess960_materialize | 0.97x | 1.64x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.99x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 20.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.67x | 1.79x | 1.32x | 2.39x | 2.07x | 10/10 |
| corpus_materialize | 0.99x | 2.01x | 1.74x | 1.23x | 2.84x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.96x | 2.38x | 3.18x | 2.35x | 2.99x | 1.68x | 10/10 |
| replay_trusted_native | 0.96x | 0.73x | 1.21x | 1.44x | 2.96x | 3.42x | 8/10 |
| replay_checked_native | 0.97x | 2.49x | 0.94x | 5.45x | 6.24x | 7.72x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
