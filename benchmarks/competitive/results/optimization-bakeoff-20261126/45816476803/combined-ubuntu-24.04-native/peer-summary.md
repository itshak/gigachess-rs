| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.20x | 1.53x | 1.47x | 2.33x | 1.78x | 30/30 |
| materialize | 1.02x | 2.14x | 1.68x | 1.11x | 2.12x | 1.44x | 28/30 |
| perft_bulk_d4 | 0.98x | 3.69x | 1.46x | 1.19x | 2.32x | 1.99x | 30/30 |
| perft_full_d3 | 1.02x | 1.26x | 1.33x | 1.00x | 1.90x | 1.60x | 26/30 |
| fen_parse | 1.01x | 1.46x | 1.62x | 1.22x | 2.05x | 1.30x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.57x | 1.84x | 27.21x | 2.18x | 30/30 |
| snapshot | 0.99x | 1.88x | 1.25x | 3.79x | 1.24x | 1.24x | 30/30 |
| san_parse_all | 0.99x | 2.25x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.49x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.82x | 1.20x | 4.37x | 1.38x | 1.60x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 103.57x | 138.21x | 215.88x | 147.04x | 5/5 |
| chess960_materialize | 1.00x | 1.81x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.90x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 15.65x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.99x | 1.60x | 1.19x | 2.15x | 1.86x | 9/10 |
| corpus_materialize | 1.00x | 1.84x | 1.68x | 1.17x | 2.67x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.32x | 3.12x | 2.32x | 2.90x | 1.65x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.57x | 1.73x | 2.98x | 3.63x | 9/10 |
| replay_checked_native | 0.99x | 2.63x | 1.00x | 5.68x | 6.50x | 7.68x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
