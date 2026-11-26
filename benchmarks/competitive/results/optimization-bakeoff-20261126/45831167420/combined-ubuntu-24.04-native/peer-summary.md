| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.69x | 1.71x | 1.63x | 2.60x | 1.98x | 30/30 |
| materialize | 1.04x | 2.14x | 1.78x | 1.16x | 2.21x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.10x | 4.14x | 1.62x | 1.33x | 2.56x | 2.19x | 30/30 |
| perft_full_d3 | 1.00x | 1.32x | 1.36x | 1.03x | 1.94x | 1.66x | 26/30 |
| fen_parse | 0.88x | 1.30x | 1.37x | 1.12x | 1.74x | 1.21x | 30/30 |
| fen_write_native_ep | 1.00x | 4.16x | 9.35x | 1.85x | 27.64x | 2.34x | 30/30 |
| snapshot | 1.01x | 1.89x | 1.25x | 3.67x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.23x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.70x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.95x | 1.27x | 4.56x | 1.47x | 1.66x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 104.17x | 152.30x | 226.58x | 159.94x | 5/5 |
| chess960_materialize | 1.06x | 1.97x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.19x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.39x | 1.77x | 1.30x | 2.35x | 2.04x | 9/10 |
| corpus_materialize | 1.05x | 1.89x | 1.76x | 1.23x | 2.80x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 3.11x | 2.31x | 2.88x | 1.65x | 10/10 |
| replay_trusted_native | 1.01x | 1.17x | 1.97x | 2.31x | 3.95x | 4.79x | 10/10 |
| replay_checked_native | 0.99x | 2.74x | 1.00x | 6.00x | 6.83x | 7.97x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
