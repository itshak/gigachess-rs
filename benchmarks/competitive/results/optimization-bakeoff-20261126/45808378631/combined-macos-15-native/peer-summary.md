| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.44x | 1.36x | 1.57x | 1.95x | 1.90x | 29/30 |
| materialize | 0.96x | 2.03x | 1.50x | 1.06x | 2.19x | 1.53x | 27/30 |
| perft_bulk_d4 | 0.73x | 3.48x | 1.26x | 1.07x | 1.62x | 1.62x | 28/30 |
| perft_full_d3 | 0.96x | 1.69x | 1.81x | 1.38x | 2.04x | 2.03x | 30/30 |
| fen_parse | 1.02x | 1.21x | 1.36x | 1.38x | 1.57x | 1.40x | 30/30 |
| fen_write_native_ep | 0.94x | 3.75x | 8.29x | 1.73x | 27.37x | 2.13x | 30/30 |
| snapshot | 1.07x | 1.69x | 1.23x | 2.33x | 1.12x | 1.04x | 27/30 |
| san_parse_all | 0.92x | 1.91x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.22x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.03x | 1.04x | 5.29x | 1.08x | 1.26x | 23/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.92x | 1.19x | 89.33x | 173.56x | 210.14x | 186.97x | 5/5 |
| chess960_materialize | 1.09x | 1.94x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.57x | 2.66x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 42.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.13x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 21.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.65x | 5.43x | 1.60x | 1.19x | 2.06x | 2.29x | 10/10 |
| corpus_materialize | 0.80x | 1.43x | 1.46x | 1.06x | 1.88x | 1.33x | 9/10 |
| corpus_uci_sorted | 0.74x | 1.89x | 2.56x | 1.88x | 2.29x | 1.25x | 10/10 |
| replay_trusted_native | 0.82x | 0.64x | 1.56x | 1.55x | 1.64x | 1.73x | 8/10 |
| replay_checked_native | 0.94x | 2.23x | 1.01x | 7.12x | 5.96x | 8.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
