| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.57x | 1.41x | 1.54x | 1.83x | 1.69x | 27/30 |
| materialize | 1.05x | 2.03x | 1.49x | 1.10x | 2.32x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.97x | 3.72x | 1.29x | 1.12x | 1.76x | 1.68x | 28/30 |
| perft_full_d3 | 0.95x | 1.55x | 1.72x | 1.37x | 2.01x | 2.17x | 30/30 |
| fen_parse | 0.97x | 1.30x | 1.37x | 1.32x | 1.73x | 1.49x | 30/30 |
| fen_write_native_ep | 0.96x | 3.81x | 8.05x | 1.72x | 24.86x | 2.24x | 30/30 |
| snapshot | 1.09x | 1.64x | 1.05x | 2.39x | 1.03x | 1.22x | 22/30 |
| san_parse_all | 0.93x | 1.91x | N/A | 3.75x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.67x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.13x | 1.15x | 5.39x | 1.11x | 1.23x | 28/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.17x | 83.37x | 178.26x | 193.72x | 181.24x | 5/5 |
| chess960_materialize | 0.83x | 1.59x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.66x | 1.16x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 47.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.76x | 0.91x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.09x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.71x | 4.32x | 1.47x | 0.97x | 1.79x | 1.84x | 9/10 |
| corpus_materialize | 0.88x | 1.38x | 1.38x | 1.02x | 1.75x | 1.20x | 9/10 |
| corpus_uci_sorted | 0.89x | 1.76x | 2.75x | 1.90x | 2.34x | 1.30x | 10/10 |
| replay_trusted_native | 1.07x | 0.61x | 1.52x | 1.55x | 1.69x | 1.82x | 8/10 |
| replay_checked_native | 0.96x | 2.18x | 1.05x | 8.33x | 6.84x | 9.20x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
