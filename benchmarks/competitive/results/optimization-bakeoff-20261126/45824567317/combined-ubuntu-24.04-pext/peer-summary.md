| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.46x | 1.55x | 1.61x | 2.51x | 1.79x | 29/30 |
| materialize | 1.01x | 2.21x | 1.71x | 1.18x | 2.21x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.94x | 1.52x | 1.27x | 2.47x | 1.99x | 30/30 |
| perft_full_d3 | 0.98x | 1.22x | 1.27x | 0.97x | 1.83x | 1.55x | 25/30 |
| fen_parse | 1.24x | 1.54x | 1.63x | 1.26x | 2.04x | 1.49x | 30/30 |
| fen_write_native_ep | 1.00x | 4.14x | 9.60x | 1.89x | 27.64x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.25x | 3.79x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.36x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.63x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.91x | 1.24x | 4.84x | 1.45x | 1.64x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 98.99x | 143.31x | 216.05x | 161.75x | 5/5 |
| chess960_materialize | 0.99x | 1.74x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 15.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.28x | 1.64x | 1.27x | 2.30x | 1.89x | 9/10 |
| corpus_materialize | 1.00x | 1.93x | 1.75x | 1.23x | 2.80x | 1.49x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.33x | 3.14x | 2.34x | 2.92x | 1.65x | 10/10 |
| replay_trusted_native | 1.06x | 1.16x | 2.01x | 2.29x | 3.90x | 4.63x | 10/10 |
| replay_checked_native | 1.07x | 2.85x | 1.13x | 6.39x | 7.23x | 8.33x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
