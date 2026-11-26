| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.51x | 1.92x | 1.63x | 2.24x | 1.96x | 30/30 |
| materialize | 1.02x | 2.15x | 1.43x | 1.57x | 2.45x | 1.67x | 30/30 |
| perft_bulk_d4 | 1.09x | 3.86x | 1.56x | 1.20x | 2.16x | 1.87x | 29/30 |
| perft_full_d3 | 1.23x | 1.72x | 1.87x | 1.72x | 2.44x | 2.00x | 30/30 |
| fen_parse | 0.98x | 1.20x | 1.51x | 1.32x | 1.81x | 1.41x | 30/30 |
| fen_write_native_ep | 0.98x | 3.89x | 7.66x | 1.69x | 29.86x | 2.14x | 30/30 |
| snapshot | 1.01x | 2.04x | 1.11x | 4.67x | 1.07x | 1.07x | 25/30 |
| san_parse_all | 1.01x | 1.79x | N/A | 5.45x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.76x | N/A | 4.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.09x | 1.39x | 10.69x | 1.35x | 1.67x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.00x | 83.39x | 163.16x | 217.69x | 178.83x | 5/5 |
| chess960_materialize | 1.01x | 1.78x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.54x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 39.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.17x | 1.22x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.21x | 32.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.59x | 1.99x | 0.96x | 2.07x | 2.03x | 9/10 |
| corpus_materialize | 0.97x | 1.37x | 1.20x | 1.06x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.80x | 2.27x | 1.92x | 2.14x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.84x | 1.54x | 1.63x | 2.28x | 2.45x | 9/10 |
| replay_checked_native | 0.98x | 2.27x | 1.16x | 9.18x | 9.04x | 10.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
