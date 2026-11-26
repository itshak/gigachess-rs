| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.06x | 1.56x | 1.67x | 2.02x | 2.01x | 30/30 |
| materialize | 0.98x | 1.90x | 1.53x | 1.32x | 2.10x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.86x | 3.30x | 1.37x | 1.23x | 1.73x | 1.78x | 30/30 |
| perft_full_d3 | 0.76x | 1.12x | 1.21x | 1.04x | 1.50x | 1.40x | 26/30 |
| fen_parse | 0.98x | 1.21x | 1.31x | 1.23x | 1.72x | 1.37x | 30/30 |
| fen_write_native_ep | 0.95x | 4.31x | 8.74x | 1.51x | 28.24x | 2.33x | 30/30 |
| snapshot | 1.56x | 1.74x | 1.33x | 2.25x | 1.23x | 1.46x | 30/30 |
| san_parse_all | 0.94x | 1.76x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.82x | 10.08x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.08x | 1.24x | 3.49x | 1.03x | 1.05x | 23/30 |
| make_unmake_full | 0.80x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.24x | 79.60x | 135.72x | 185.53x | 151.43x | 5/5 |
| chess960_materialize | 0.97x | 1.64x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 2.60x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 26.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.15x | 21.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 4.14x | 1.58x | 1.26x | 1.86x | 2.13x | 10/10 |
| corpus_materialize | 0.98x | 1.59x | 1.49x | 1.21x | 1.87x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.16x | 2.96x | 2.14x | 2.92x | 1.50x | 10/10 |
| replay_trusted_native | 0.85x | 0.53x | 1.45x | 1.71x | 1.85x | 1.94x | 8/10 |
| replay_checked_native | 0.88x | 2.05x | 0.99x | 6.68x | 5.93x | 7.79x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
