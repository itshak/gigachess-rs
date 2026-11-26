| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.56x | 1.57x | 1.59x | 2.55x | 1.81x | 30/30 |
| materialize | 0.98x | 2.11x | 1.70x | 1.14x | 2.19x | 1.43x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.97x | 1.54x | 1.29x | 2.48x | 2.01x | 30/30 |
| perft_full_d3 | 1.08x | 1.29x | 1.34x | 1.01x | 1.93x | 1.52x | 26/30 |
| fen_parse | 1.02x | 1.47x | 1.58x | 1.24x | 2.54x | 1.49x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.60x | 1.85x | 28.30x | 2.09x | 30/30 |
| snapshot | 0.53x | 1.00x | 0.66x | 1.99x | 0.66x | 0.66x | 9/30 |
| san_parse_all | 1.02x | 2.36x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.71x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.77x | 1.18x | 4.30x | 1.35x | 1.54x | 29/30 |
| make_unmake_full | 0.80x | N/A | N/A | 0.98x | N/A | N/A | 2/6 |
| startpos | 1.00x | 1.20x | 101.93x | 141.76x | 243.70x | 163.87x | 5/5 |
| chess960_materialize | 1.03x | 1.83x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.93x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.91x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 15.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.24x | 1.65x | 1.26x | 2.29x | 1.88x | 9/10 |
| corpus_materialize | 1.00x | 1.86x | 1.70x | 1.22x | 2.80x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.33x | 3.17x | 2.34x | 2.93x | 1.65x | 10/10 |
| replay_trusted_native | 0.98x | 0.74x | 1.33x | 1.48x | 2.53x | 2.98x | 8/10 |
| replay_checked_native | 1.06x | 2.54x | 0.99x | 5.91x | 6.56x | 7.33x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
