| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.35x | 1.45x | 1.54x | 1.94x | 1.77x | 29/30 |
| materialize | 0.99x | 1.97x | 1.63x | 1.03x | 2.51x | 1.48x | 28/30 |
| perft_bulk_d4 | 0.90x | 3.48x | 1.26x | 0.98x | 1.58x | 1.46x | 25/30 |
| perft_full_d3 | 1.02x | 1.66x | 1.79x | 1.44x | 2.08x | 2.07x | 30/30 |
| fen_parse | 0.99x | 1.25x | 1.37x | 1.35x | 1.61x | 1.39x | 30/30 |
| fen_write_native_ep | 0.97x | 4.29x | 9.18x | 1.77x | 38.54x | 2.25x | 30/30 |
| snapshot | 0.94x | 1.63x | 1.09x | 2.32x | 1.11x | 1.00x | 28/30 |
| san_parse_all | 1.06x | 1.83x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.54x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.09x | 1.15x | 5.47x | 1.11x | 1.28x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.20x | 68.60x | 139.62x | 159.12x | 131.46x | 5/5 |
| chess960_materialize | 1.01x | 1.63x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.66x | 1.23x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 37.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 18.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.97x | 1.65x | 0.99x | 1.93x | 2.26x | 9/10 |
| corpus_materialize | 1.04x | 1.42x | 1.36x | 1.12x | 1.88x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.07x | 1.80x | 2.56x | 1.79x | 2.35x | 1.36x | 10/10 |
| replay_trusted_native | 1.08x | 0.64x | 1.65x | 1.67x | 1.79x | 1.80x | 8/10 |
| replay_checked_native | 1.02x | 2.36x | 1.15x | 8.63x | 7.89x | 9.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
