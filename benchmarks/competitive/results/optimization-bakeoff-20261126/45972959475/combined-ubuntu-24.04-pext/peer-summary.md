| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 5.01x | 1.73x | 1.77x | 2.76x | 2.02x | 30/30 |
| materialize | 1.01x | 2.21x | 1.77x | 1.20x | 2.28x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.30x | 1.66x | 1.39x | 2.68x | 2.17x | 30/30 |
| perft_full_d3 | 1.01x | 1.53x | 1.57x | 1.20x | 2.28x | 1.81x | 30/30 |
| fen_parse | 0.93x | 1.27x | 1.43x | 1.07x | 1.86x | 1.32x | 30/30 |
| fen_write_native_ep | 1.01x | 4.16x | 9.59x | 1.86x | 27.90x | 2.22x | 30/30 |
| snapshot | 1.00x | 1.89x | 1.25x | 3.81x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.25x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.18x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.00x | 1.30x | 4.64x | 1.52x | 1.72x | 30/30 |
| make_unmake_full | 1.10x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 102.57x | 137.03x | 219.31x | 164.27x | 5/5 |
| chess960_materialize | 1.02x | 2.00x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.16x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.94x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.64x | 1.80x | 1.37x | 2.51x | 2.05x | 10/10 |
| corpus_materialize | 1.00x | 1.95x | 1.80x | 1.27x | 2.96x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.38x | 3.16x | 2.36x | 2.96x | 1.67x | 10/10 |
| replay_trusted_native | 0.93x | 1.10x | 1.94x | 2.15x | 3.68x | 4.34x | 10/10 |
| replay_checked_native | 0.99x | 2.86x | 1.11x | 6.23x | 7.06x | 8.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
