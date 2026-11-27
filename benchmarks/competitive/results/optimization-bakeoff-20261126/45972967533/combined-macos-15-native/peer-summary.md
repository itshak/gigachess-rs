| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.22x | 5.26x | 1.87x | 1.92x | 2.95x | 2.24x | 30/30 |
| materialize | 1.00x | 1.99x | 1.78x | 1.06x | 2.08x | 1.37x | 28/30 |
| perft_bulk_d4 | 1.14x | 3.91x | 1.58x | 1.31x | 1.91x | 1.83x | 30/30 |
| perft_full_d3 | 1.26x | 1.75x | 2.10x | 1.48x | 2.15x | 2.42x | 30/30 |
| fen_parse | 1.08x | 1.18x | 1.28x | 1.29x | 1.64x | 1.36x | 28/30 |
| fen_write_native_ep | 1.07x | 4.08x | 8.86x | 1.60x | 28.31x | 5.41x | 30/30 |
| snapshot | 0.91x | 1.52x | 1.04x | 2.19x | 1.01x | 0.88x | 21/30 |
| san_parse_all | 1.06x | 1.87x | N/A | 3.71x | N/A | N/A | 12/12 |
| san_render_native | 1.18x | 13.44x | N/A | 3.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.18x | 1.23x | 5.87x | 1.12x | 1.25x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.63x | 0.78x | 66.26x | 134.78x | 177.65x | 139.82x | 4/5 |
| chess960_materialize | 0.87x | 1.91x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.13x | 3.60x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 37.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.88x | 0.91x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.14x | 19.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.50x | 1.89x | 1.20x | 2.30x | 2.37x | 10/10 |
| corpus_materialize | 1.08x | 1.54x | 1.37x | 1.15x | 1.89x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.14x | 1.60x | 2.39x | 2.27x | 2.35x | 1.47x | 10/10 |
| replay_trusted_native | 0.97x | 0.60x | 1.80x | 1.62x | 1.81x | 1.70x | 8/10 |
| replay_checked_native | 1.24x | 3.04x | 1.22x | 11.73x | 9.37x | 13.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
