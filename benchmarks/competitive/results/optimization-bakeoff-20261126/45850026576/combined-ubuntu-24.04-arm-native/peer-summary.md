| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.68x | 1.80x | 2.20x | 2.17x | 30/30 |
| materialize | 1.00x | 1.95x | 1.56x | 1.36x | 2.17x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.90x | 1.58x | 1.43x | 2.03x | 2.07x | 30/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.60x | 1.33x | 1.96x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.21x | 1.31x | 1.26x | 1.72x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.46x | 9.35x | 1.60x | 30.20x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.04x | 1.47x | 1.03x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.88x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.04x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 76.31x | 129.33x | 176.24x | 143.76x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.00x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 24.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.50x | 1.70x | 1.37x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.52x | 1.24x | 1.90x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.12x | 2.93x | 2.10x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.61x | 1.62x | 1.95x | 2.19x | 2.23x | 8/10 |
| replay_checked_native | 0.98x | 2.14x | 1.03x | 6.95x | 6.20x | 8.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
