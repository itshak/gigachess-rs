| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.67x | 1.52x | 1.32x | 1.77x | 1.26x | 28/30 |
| materialize | 0.98x | 2.16x | 1.83x | 1.79x | 2.73x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.95x | 1.56x | 1.17x | 1.96x | 1.74x | 30/30 |
| perft_full_d3 | 1.01x | 1.53x | 1.80x | 1.60x | 2.18x | 1.80x | 30/30 |
| fen_parse | 0.95x | 1.68x | 1.36x | 1.13x | 1.45x | 1.19x | 30/30 |
| fen_write_native_ep | 1.55x | 3.04x | 9.96x | 1.77x | 27.76x | 2.32x | 30/30 |
| snapshot | 1.00x | 1.21x | 1.00x | 2.69x | 1.00x | 1.00x | 19/30 |
| san_parse_all | 1.00x | 1.88x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.36x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.81x | 1.16x | 3.41x | 1.10x | 1.29x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.29x | 60.91x | 105.64x | 122.59x | 105.14x | 5/5 |
| chess960_materialize | 0.99x | 1.64x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.89x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 42.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 22.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.62x | 2.30x | 1.26x | 2.35x | 3.17x | 10/10 |
| corpus_materialize | 0.99x | 1.52x | 1.41x | 1.14x | 1.84x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.94x | 2.99x | 2.21x | 2.92x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.69x | 1.35x | 1.31x | 1.79x | 1.65x | 8/10 |
| replay_checked_native | 0.99x | 1.98x | 1.11x | 8.41x | 8.18x | 9.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
