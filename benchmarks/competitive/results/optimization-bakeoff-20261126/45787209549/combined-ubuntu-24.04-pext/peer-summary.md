| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.79x | 1.56x | 2.39x | 1.81x | 30/30 |
| materialize | 1.00x | 2.19x | 1.67x | 1.73x | 2.64x | 1.82x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.05x | 1.61x | 1.27x | 2.50x | 1.97x | 30/30 |
| perft_full_d3 | 0.97x | 1.47x | 1.70x | 1.64x | 2.11x | 1.78x | 30/30 |
| fen_parse | 0.97x | 1.29x | 1.29x | 1.08x | 1.46x | 1.15x | 30/30 |
| fen_write_native_ep | 1.01x | 5.20x | 10.63x | 1.92x | 30.83x | 2.54x | 30/30 |
| snapshot | 1.41x | 1.42x | 1.15x | 3.23x | 1.15x | 1.16x | 30/30 |
| san_parse_all | 1.16x | 1.98x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.06x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.67x | 1.07x | 3.12x | 1.08x | 1.29x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.22x | 1.21x | 82.22x | 152.44x | 186.74x | 157.98x | 5/5 |
| chess960_materialize | 0.93x | 1.64x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.40x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 38.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.82x | 2.51x | 1.41x | 3.16x | 2.80x | 10/10 |
| corpus_materialize | 1.02x | 1.48x | 1.36x | 1.14x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.33x | 3.16x | 2.33x | 3.07x | 1.57x | 10/10 |
| replay_trusted_native | 0.98x | 0.70x | 1.22x | 1.25x | 1.76x | 2.24x | 7/10 |
| replay_checked_native | 0.99x | 2.06x | 1.01x | 8.48x | 7.95x | 9.39x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
