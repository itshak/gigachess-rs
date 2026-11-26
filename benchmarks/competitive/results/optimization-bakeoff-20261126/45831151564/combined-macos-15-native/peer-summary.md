| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.89x | 3.97x | 1.32x | 1.49x | 1.84x | 1.76x | 29/30 |
| materialize | 0.99x | 1.86x | 1.47x | 1.11x | 2.22x | 1.40x | 29/30 |
| perft_bulk_d4 | 0.96x | 3.86x | 1.41x | 1.17x | 1.77x | 1.68x | 28/30 |
| perft_full_d3 | 0.99x | 1.69x | 1.87x | 1.40x | 2.02x | 2.08x | 30/30 |
| fen_parse | 0.97x | 1.28x | 1.43x | 1.35x | 1.69x | 1.42x | 30/30 |
| fen_write_native_ep | 1.07x | 4.31x | 8.72x | 1.73x | 27.34x | 2.18x | 30/30 |
| snapshot | 1.08x | 1.80x | 1.18x | 2.71x | 1.29x | 1.07x | 29/30 |
| san_parse_all | 0.96x | 1.68x | N/A | 3.67x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.16x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.16x | 1.21x | 5.92x | 1.17x | 1.38x | 29/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.24x | 1.19x | 67.75x | 141.94x | 171.75x | 142.27x | 5/5 |
| chess960_materialize | 1.03x | 1.59x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.46x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 37.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.88x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 4.98x | 1.55x | 1.07x | 1.82x | 2.41x | 10/10 |
| corpus_materialize | 0.98x | 1.54x | 1.40x | 1.11x | 2.01x | 1.11x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.74x | 2.45x | 2.13x | 2.23x | 1.41x | 10/10 |
| replay_trusted_native | 1.19x | 0.68x | 1.72x | 1.58x | 1.73x | 1.93x | 8/10 |
| replay_checked_native | 1.14x | 2.60x | 1.15x | 8.44x | 7.39x | 9.95x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
