| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.29x | 1.57x | 1.50x | 2.35x | 1.82x | 30/30 |
| materialize | 1.01x | 2.01x | 1.65x | 1.22x | 2.04x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.79x | 1.49x | 1.22x | 2.36x | 2.02x | 30/30 |
| perft_full_d3 | 1.01x | 1.29x | 1.32x | 1.04x | 1.89x | 1.62x | 27/30 |
| fen_parse | 0.97x | 1.21x | 1.31x | 1.01x | 1.69x | 1.20x | 26/30 |
| fen_write_native_ep | 1.00x | 4.13x | 10.18x | 1.86x | 28.39x | 2.08x | 30/30 |
| snapshot | 1.23x | 1.89x | 1.25x | 3.65x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.03x | 2.29x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.20x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.50x | 1.77x | 1.15x | 4.11x | 1.35x | 1.53x | 29/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.20x | 97.67x | 138.12x | 216.36x | 159.58x | 5/5 |
| chess960_materialize | 1.08x | 1.86x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.83x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 15.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 4.00x | 1.62x | 1.19x | 2.16x | 1.86x | 9/10 |
| corpus_materialize | 0.99x | 1.81x | 1.70x | 1.30x | 2.71x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.17x | 2.35x | 2.98x | 1.68x | 10/10 |
| replay_trusted_native | 1.11x | 0.87x | 1.57x | 1.74x | 2.93x | 3.60x | 9/10 |
| replay_checked_native | 1.03x | 2.62x | 1.01x | 6.17x | 6.29x | 7.63x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
