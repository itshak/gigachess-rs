| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.15x | 1.50x | 1.44x | 2.19x | 1.79x | 30/30 |
| materialize | 1.00x | 1.85x | 1.59x | 1.19x | 2.18x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.73x | 1.49x | 1.25x | 2.23x | 1.86x | 30/30 |
| perft_full_d3 | 0.79x | 1.03x | 1.14x | 1.01x | 1.46x | 1.47x | 22/30 |
| fen_parse | 1.11x | 1.40x | 1.58x | 1.23x | 2.29x | 1.36x | 30/30 |
| fen_write_native_ep | 1.03x | 4.00x | 10.08x | 1.74x | 29.06x | 2.10x | 30/30 |
| snapshot | 0.75x | 1.58x | 1.20x | 2.48x | 1.63x | 1.04x | 30/30 |
| san_parse_all | 0.96x | 2.04x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.03x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.89x | 1.09x | 3.68x | 1.19x | 1.38x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.19x | 74.16x | 130.95x | 218.23x | 147.67x | 5/5 |
| chess960_materialize | 0.99x | 1.47x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.46x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 28.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 21.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.28x | 1.76x | 1.33x | 2.54x | 2.35x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.56x | 1.17x | 2.03x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.38x | 3.19x | 2.41x | 3.06x | 1.81x | 10/10 |
| replay_trusted_native | 1.02x | 0.70x | 1.29x | 1.59x | 2.40x | 2.43x | 8/10 |
| replay_checked_native | 1.00x | 2.34x | 1.00x | 7.84x | 7.54x | 9.04x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
