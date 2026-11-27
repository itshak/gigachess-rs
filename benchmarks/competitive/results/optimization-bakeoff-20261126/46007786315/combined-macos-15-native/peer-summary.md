| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.39x | 1.99x | 2.04x | 2.65x | 2.23x | 30/30 |
| materialize | 0.99x | 2.01x | 1.37x | 1.02x | 2.13x | 1.35x | 27/30 |
| perft_bulk_d4 | 1.08x | 4.41x | 1.71x | 1.30x | 2.12x | 1.97x | 29/30 |
| perft_full_d3 | 1.01x | 1.60x | 1.76x | 1.44x | 2.08x | 2.20x | 30/30 |
| fen_parse | 0.97x | 1.29x | 1.41x | 1.41x | 1.75x | 1.47x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 8.95x | 1.64x | 28.99x | 2.23x | 30/30 |
| snapshot | 1.06x | 1.46x | 1.13x | 2.23x | 1.08x | 0.95x | 20/30 |
| san_parse_all | 0.96x | 1.80x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 11.92x | N/A | 2.79x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.27x | 1.30x | 6.38x | 1.24x | 1.46x | 29/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 0.72x | 0.93x | 51.07x | 113.55x | 117.39x | 118.47x | 4/5 |
| chess960_materialize | 0.88x | 1.93x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 3.14x | 1.46x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 35.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.89x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 23.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.03x | 2.18x | 1.48x | 2.76x | 2.59x | 10/10 |
| corpus_materialize | 0.92x | 1.50x | 1.47x | 1.38x | 2.16x | 1.44x | 10/10 |
| corpus_uci_sorted | 0.88x | 1.85x | 2.66x | 2.00x | 2.60x | 1.33x | 10/10 |
| replay_trusted_native | 0.90x | 0.64x | 1.70x | 1.43x | 1.86x | 1.76x | 8/10 |
| replay_checked_native | 0.77x | 2.78x | 1.11x | 7.87x | 7.56x | 9.51x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
