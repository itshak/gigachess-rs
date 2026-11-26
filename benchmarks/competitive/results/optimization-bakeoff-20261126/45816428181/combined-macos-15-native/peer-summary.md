| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.31x | 1.44x | 1.51x | 1.96x | 1.83x | 29/30 |
| materialize | 0.97x | 2.07x | 1.41x | 1.06x | 2.31x | 1.35x | 27/30 |
| perft_bulk_d4 | 0.84x | 3.62x | 1.39x | 1.15x | 1.77x | 1.57x | 29/30 |
| perft_full_d3 | 1.00x | 1.56x | 1.87x | 1.41x | 2.28x | 2.21x | 30/30 |
| fen_parse | 0.94x | 1.24x | 1.34x | 1.27x | 1.51x | 1.35x | 29/30 |
| fen_write_native_ep | 1.05x | 4.61x | 9.94x | 1.89x | 32.16x | 2.33x | 30/30 |
| snapshot | 1.20x | 1.47x | 1.03x | 2.20x | 0.94x | 0.86x | 20/30 |
| san_parse_all | 0.96x | 1.79x | N/A | 3.53x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.70x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.13x | 1.24x | 6.01x | 1.18x | 1.42x | 28/30 |
| make_unmake_full | 1.11x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.20x | 1.21x | 88.44x | 188.76x | 238.44x | 191.87x | 5/5 |
| chess960_materialize | 1.16x | 1.93x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.81x | 2.45x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 36.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 17.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.71x | 4.64x | 1.73x | 1.04x | 2.02x | 2.20x | 9/10 |
| corpus_materialize | 0.92x | 1.46x | 1.28x | 1.07x | 1.81x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.63x | 2.34x | 1.95x | 2.04x | 1.28x | 10/10 |
| replay_trusted_native | 1.12x | 0.63x | 1.67x | 1.76x | 1.50x | 1.78x | 8/10 |
| replay_checked_native | 1.09x | 2.07x | 0.99x | 9.01x | 6.91x | 9.15x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
