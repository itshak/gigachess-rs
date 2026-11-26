| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.18x | 1.53x | 1.47x | 2.31x | 1.77x | 30/30 |
| materialize | 0.99x | 2.01x | 1.65x | 1.14x | 2.08x | 1.48x | 29/30 |
| perft_bulk_d4 | 0.96x | 3.74x | 1.47x | 1.21x | 2.33x | 2.00x | 30/30 |
| perft_full_d3 | 0.99x | 1.28x | 1.32x | 1.00x | 1.88x | 1.95x | 26/30 |
| fen_parse | 1.04x | 1.31x | 1.44x | 1.08x | 1.84x | 1.27x | 28/30 |
| fen_write_native_ep | 1.00x | 4.26x | 9.58x | 1.83x | 27.92x | 2.08x | 30/30 |
| snapshot | 1.00x | 1.94x | 1.25x | 3.79x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.25x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.97x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.16x | 1.90x | 1.29x | 4.49x | 1.43x | 1.63x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 99.41x | 134.79x | 208.86x | 154.76x | 5/5 |
| chess960_materialize | 1.07x | 1.88x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.00x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 15.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 4.00x | 1.60x | 1.18x | 2.14x | 1.86x | 9/10 |
| corpus_materialize | 0.98x | 1.80x | 1.69x | 1.17x | 2.73x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.32x | 3.13x | 2.32x | 2.96x | 1.65x | 10/10 |
| replay_trusted_native | 1.35x | 1.07x | 1.84x | 2.10x | 3.62x | 4.41x | 10/10 |
| replay_checked_native | 1.06x | 2.64x | 1.01x | 5.78x | 6.66x | 8.23x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
