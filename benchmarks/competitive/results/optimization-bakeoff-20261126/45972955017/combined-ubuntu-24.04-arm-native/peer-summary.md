| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.33x | 2.14x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.56x | 1.43x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.34x | 1.27x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.38x | 9.24x | 1.57x | 29.88x | 2.39x | 30/30 |
| snapshot | 1.02x | 1.15x | 1.02x | 1.49x | 1.02x | 0.94x | 23/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.46x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.26x | 1.48x | 4.03x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.41x | 125.39x | 176.53x | 143.78x | 5/5 |
| chess960_materialize | 1.00x | 1.70x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.99x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.42x | 1.69x | 1.35x | 1.99x | 2.24x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.52x | 1.22x | 1.89x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.66x | 2.01x | 2.26x | 2.29x | 8/10 |
| replay_checked_native | 1.00x | 2.32x | 1.13x | 7.64x | 6.83x | 8.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
