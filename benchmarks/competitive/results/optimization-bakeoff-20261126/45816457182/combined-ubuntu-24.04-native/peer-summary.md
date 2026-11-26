| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.91x | 1.69x | 1.42x | 2.14x | 1.70x | 30/30 |
| materialize | 1.00x | 2.09x | 1.60x | 1.63x | 2.46x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.28x | 1.34x | 1.02x | 1.99x | 1.60x | 28/30 |
| perft_full_d3 | 0.99x | 1.40x | 1.71x | 1.56x | 2.01x | 1.76x | 30/30 |
| fen_parse | 1.01x | 1.48x | 1.37x | 1.16x | 1.47x | 1.25x | 30/30 |
| fen_write_native_ep | 1.03x | 4.91x | 10.06x | 1.91x | 28.11x | 2.57x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 20/30 |
| san_parse_all | 1.11x | 1.88x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.28x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.88x | 1.19x | 3.57x | 1.19x | 1.47x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 98.94x | 181.75x | 214.86x | 194.73x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.35x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 18.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.18x | 1.59x | 0.87x | 1.90x | 1.76x | 8/10 |
| corpus_materialize | 1.01x | 1.35x | 1.25x | 1.03x | 1.64x | 1.13x | 9/10 |
| corpus_uci_sorted | 0.99x | 2.30x | 3.01x | 2.24x | 2.91x | 1.56x | 10/10 |
| replay_trusted_native | 0.96x | 0.79x | 1.44x | 1.42x | 1.92x | 2.30x | 8/10 |
| replay_checked_native | 0.99x | 2.11x | 1.09x | 8.59x | 8.26x | 9.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
