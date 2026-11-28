| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.73x | 1.74x | 1.66x | 2.63x | 2.02x | 30/30 |
| materialize | 1.00x | 2.15x | 1.83x | 1.17x | 2.25x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.21x | 1.65x | 1.35x | 2.60x | 2.23x | 30/30 |
| perft_full_d3 | 1.34x | 2.01x | 2.08x | 1.60x | 2.99x | 2.52x | 30/30 |
| fen_parse | 0.85x | 1.18x | 1.26x | 1.02x | 1.84x | 1.00x | 25/30 |
| fen_write_native_ep | 1.01x | 4.04x | 9.80x | 1.82x | 26.86x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.27x | 3.75x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.25x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.44x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.00x | 1.32x | 4.78x | 1.52x | 1.75x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 103.31x | 156.72x | 232.52x | 143.37x | 5/5 |
| chess960_materialize | 1.01x | 1.96x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.12x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.24x | 1.26x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.67x | 1.87x | 1.37x | 2.51x | 2.17x | 10/10 |
| corpus_materialize | 1.01x | 1.93x | 1.79x | 1.25x | 2.87x | 1.58x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.37x | 3.20x | 2.36x | 2.99x | 1.69x | 10/10 |
| replay_trusted_native | 1.01x | 1.14x | 2.04x | 2.30x | 3.89x | 4.64x | 10/10 |
| replay_checked_native | 1.01x | 2.95x | 1.14x | 6.35x | 7.40x | 8.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
