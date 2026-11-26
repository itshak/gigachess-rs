| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 5.16x | 1.76x | 1.76x | 2.46x | 2.00x | 30/30 |
| materialize | 0.92x | 2.07x | 1.65x | 1.03x | 2.24x | 1.37x | 27/30 |
| perft_bulk_d4 | 0.99x | 3.99x | 1.55x | 1.31x | 1.97x | 1.75x | 30/30 |
| perft_full_d3 | 0.96x | 1.39x | 1.55x | 1.14x | 1.91x | 1.70x | 28/30 |
| fen_parse | 1.00x | 1.33x | 1.37x | 1.48x | 1.73x | 1.42x | 30/30 |
| fen_write_native_ep | 0.93x | 3.94x | 8.98x | 1.74x | 27.59x | 2.25x | 30/30 |
| snapshot | 0.80x | 1.51x | 0.85x | 2.14x | 0.93x | 0.90x | 19/30 |
| san_parse_all | 0.92x | 1.79x | N/A | 4.00x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.40x | N/A | 2.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.13x | 1.28x | 5.74x | 1.24x | 1.35x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.26x | N/A | N/A | 5/6 |
| startpos | 0.90x | 0.94x | 45.63x | 88.23x | 101.57x | 82.93x | 4/5 |
| chess960_materialize | 1.42x | 1.60x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.27x | 3.25x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 36.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.30x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.52x | 21.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.24x | 1.82x | 1.21x | 2.15x | 2.97x | 10/10 |
| corpus_materialize | 1.25x | 1.44x | 1.25x | 1.09x | 1.70x | 1.12x | 10/10 |
| corpus_uci_sorted | 1.18x | 1.97x | 3.03x | 2.08x | 2.59x | 1.53x | 10/10 |
| replay_trusted_native | 1.13x | 0.65x | 1.73x | 1.89x | 1.94x | 1.86x | 8/10 |
| replay_checked_native | 1.18x | 2.21x | 1.23x | 9.27x | 7.53x | 9.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
