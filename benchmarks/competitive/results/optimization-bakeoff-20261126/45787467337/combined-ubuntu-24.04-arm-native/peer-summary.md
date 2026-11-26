| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.88x | 1.42x | 1.27x | 2.00x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.52x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.16x | 1.26x | 1.06x | 1.51x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.26x | 1.34x | 1.29x | 1.80x | 1.43x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.34x | 1.57x | 29.68x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.14x | 1.13x | 1.59x | 1.14x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.90x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.12x | 1.33x | 3.60x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.47x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.16x | 67.00x | 111.92x | 151.05x | 122.12x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.75x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.82x | 1.83x | 1.49x | 2.16x | 2.38x | 10/10 |
| corpus_materialize | 1.01x | 1.59x | 1.50x | 1.23x | 1.89x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.90x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.04x | 0.53x | 1.37x | 1.67x | 1.85x | 1.90x | 8/10 |
| replay_checked_native | 1.03x | 2.09x | 1.01x | 6.81x | 6.06x | 7.89x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
