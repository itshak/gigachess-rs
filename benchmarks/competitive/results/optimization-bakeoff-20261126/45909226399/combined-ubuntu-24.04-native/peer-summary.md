| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.79x | 1.65x | 1.56x | 2.36x | 1.98x | 30/30 |
| materialize | 0.98x | 2.09x | 1.70x | 1.18x | 2.41x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.13x | 1.58x | 1.25x | 2.44x | 2.29x | 30/30 |
| perft_full_d3 | 0.99x | 1.39x | 1.66x | 1.25x | 2.12x | 2.08x | 30/30 |
| fen_parse | 0.91x | 1.29x | 1.47x | 1.16x | 1.95x | 1.33x | 29/30 |
| fen_write_native_ep | 0.96x | 3.77x | 8.27x | 1.51x | 24.48x | 1.95x | 30/30 |
| snapshot | 1.00x | 2.49x | 1.49x | 4.47x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.76x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.69x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.83x | 1.34x | 3.90x | 1.30x | 1.83x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.13x | N/A | N/A | 5/6 |
| startpos | 0.95x | 1.24x | 83.25x | 138.54x | 208.52x | 143.10x | 5/5 |
| chess960_materialize | 0.98x | 1.76x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.84x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 37.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 27.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.75x | 1.71x | 1.13x | 2.23x | 2.14x | 9/10 |
| corpus_materialize | 1.00x | 1.68x | 1.54x | 1.13x | 2.20x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.33x | 2.90x | 2.37x | 2.93x | 1.64x | 10/10 |
| replay_trusted_native | 1.00x | 0.74x | 1.41x | 1.54x | 2.48x | 2.99x | 9/10 |
| replay_checked_native | 0.99x | 2.58x | 1.17x | 9.01x | 9.16x | 11.20x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
