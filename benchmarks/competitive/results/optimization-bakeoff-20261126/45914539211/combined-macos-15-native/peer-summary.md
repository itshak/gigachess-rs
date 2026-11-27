| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.00x | 1.74x | 1.93x | 2.37x | 2.10x | 29/30 |
| materialize | 0.96x | 1.92x | 1.54x | 1.14x | 2.43x | 1.45x | 28/30 |
| perft_bulk_d4 | 1.07x | 4.38x | 1.60x | 1.26x | 1.93x | 1.88x | 29/30 |
| perft_full_d3 | 1.25x | 1.74x | 1.94x | 1.43x | 2.25x | 2.17x | 30/30 |
| fen_parse | 1.11x | 1.25x | 1.35x | 1.40x | 1.64x | 1.38x | 30/30 |
| fen_write_native_ep | 1.23x | 4.32x | 9.49x | 1.77x | 28.61x | 2.35x | 30/30 |
| snapshot | 1.01x | 1.78x | 0.96x | 2.08x | 1.14x | 0.86x | 22/30 |
| san_parse_all | 1.04x | 1.80x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 1.12x | 12.31x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.17x | 1.26x | 5.91x | 1.20x | 1.37x | 29/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.84x | 0.93x | 61.86x | 130.83x | 140.58x | 115.07x | 4/5 |
| chess960_materialize | 0.93x | 1.79x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 3.11x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 35.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 19.86x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.43x | 2.02x | 1.40x | 2.41x | 2.95x | 10/10 |
| corpus_materialize | 0.84x | 1.35x | 1.30x | 1.01x | 1.60x | 1.04x | 8/10 |
| corpus_uci_sorted | 1.05x | 1.69x | 2.38x | 1.74x | 2.28x | 1.27x | 10/10 |
| replay_trusted_native | 1.25x | 0.67x | 1.61x | 1.84x | 1.88x | 1.89x | 8/10 |
| replay_checked_native | 0.98x | 2.45x | 1.08x | 9.31x | 8.02x | 10.79x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
