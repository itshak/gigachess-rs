| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.83x | 1.47x | 1.58x | 1.91x | 1.90x | 30/30 |
| materialize | 1.00x | 1.83x | 1.44x | 1.26x | 2.03x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.53x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.08x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.24x | 1.32x | 1.26x | 1.75x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.62x | 9.29x | 1.57x | 29.29x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.22x | 0.94x | 1.56x | 0.86x | 1.02x | 18/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.79x | N/A | 2.57x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.13x | 1.29x | 3.63x | 1.08x | 1.09x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 76.58x | 130.81x | 175.40x | 141.57x | 5/5 |
| chess960_materialize | 1.01x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.64x | 1.79x | 1.44x | 2.12x | 2.31x | 10/10 |
| corpus_materialize | 0.98x | 1.54x | 1.47x | 1.17x | 1.84x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.09x | 2.89x | 2.06x | 2.75x | 1.44x | 10/10 |
| replay_trusted_native | 1.00x | 0.50x | 1.31x | 1.60x | 1.75x | 1.81x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.98x | 6.57x | 5.90x | 7.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
