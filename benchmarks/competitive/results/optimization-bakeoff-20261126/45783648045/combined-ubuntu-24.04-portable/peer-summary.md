| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.47x | 1.35x | 1.86x | 1.29x | 29/30 |
| materialize | 1.01x | 2.28x | 1.71x | 1.24x | 2.30x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.92x | 1.53x | 1.20x | 2.04x | 1.64x | 30/30 |
| perft_full_d3 | 1.00x | 1.23x | 1.48x | 1.13x | 1.71x | 1.60x | 29/30 |
| fen_parse | 0.86x | 1.52x | 1.39x | 1.09x | 1.74x | 1.19x | 30/30 |
| fen_write_native_ep | 0.94x | 1.74x | 6.04x | 1.20x | 17.54x | 1.29x | 30/30 |
| snapshot | 1.01x | 1.72x | 1.21x | 2.80x | 1.14x | 0.93x | 24/30 |
| san_parse_all | 1.00x | 2.23x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 11.33x | N/A | 2.37x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.88x | 1.18x | 4.49x | 1.12x | 1.12x | 25/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 79.17x | 104.51x | 160.81x | 111.39x | 5/5 |
| chess960_materialize | 1.01x | 1.78x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.23x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 21.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.15x | 1.77x | 1.25x | 1.92x | 1.84x | 10/10 |
| corpus_materialize | 0.99x | 1.93x | 1.78x | 1.35x | 2.89x | 1.72x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.03x | 2.91x | 2.19x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 1.06x | 0.72x | 1.45x | 1.56x | 2.28x | 2.09x | 8/10 |
| replay_checked_native | 1.04x | 2.34x | 1.04x | 5.65x | 6.59x | 7.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
