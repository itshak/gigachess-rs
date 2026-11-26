| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.52x | 1.43x | 1.30x | 1.58x | 1.26x | 29/30 |
| materialize | 0.97x | 2.08x | 1.59x | 1.41x | 2.29x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.98x | 2.74x | 1.50x | 1.16x | 1.68x | 1.62x | 30/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.65x | 1.35x | 1.80x | 1.77x | 30/30 |
| fen_parse | 0.89x | 1.38x | 1.36x | 1.07x | 1.60x | 1.18x | 29/30 |
| fen_write_native_ep | 1.04x | 3.30x | 9.91x | 1.88x | 27.62x | 2.26x | 30/30 |
| snapshot | 1.02x | 2.61x | 1.75x | 2.79x | 1.12x | 0.99x | 24/30 |
| san_parse_all | 0.98x | 1.91x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 0.91x | 10.76x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.67x | 1.09x | 3.53x | 1.13x | 1.12x | 28/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 71.83x | 124.18x | 163.69x | 123.53x | 5/5 |
| chess960_materialize | 1.01x | 1.53x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.83x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 31.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 20.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.06x | 2.12x | 1.23x | 2.04x | 2.78x | 10/10 |
| corpus_materialize | 1.03x | 1.58x | 1.49x | 1.18x | 1.89x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.78x | 2.18x | 2.69x | 1.52x | 10/10 |
| replay_trusted_native | 0.97x | 0.67x | 1.40x | 1.41x | 1.77x | 1.60x | 8/10 |
| replay_checked_native | 0.95x | 1.82x | 0.98x | 7.12x | 6.76x | 8.01x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
