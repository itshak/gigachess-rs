| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.94x | 1.70x | 1.68x | 2.50x | 2.16x | 30/30 |
| materialize | 0.99x | 2.09x | 1.40x | 1.32x | 2.48x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.13x | 1.64x | 1.32x | 2.29x | 2.04x | 30/30 |
| perft_full_d3 | 1.01x | 1.40x | 1.49x | 1.35x | 1.98x | 1.86x | 30/30 |
| fen_parse | 1.09x | 1.38x | 1.50x | 1.39x | 2.29x | 1.54x | 30/30 |
| fen_write_native_ep | 0.99x | 4.11x | 8.07x | 1.62x | 32.17x | 1.95x | 30/30 |
| snapshot | 1.06x | 1.84x | 0.99x | 4.25x | 1.00x | 1.00x | 24/30 |
| san_parse_all | 1.02x | 2.03x | N/A | 5.28x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.18x | N/A | 3.91x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 2.38x | 1.46x | 10.02x | 1.54x | 1.75x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.69x | 178.69x | 232.66x | 175.29x | 5/5 |
| chess960_materialize | 1.09x | 1.64x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.84x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.85x | 23.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.10x | 21.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.36x | 1.81x | 1.60x | 2.46x | 2.86x | 10/10 |
| corpus_materialize | 0.91x | 1.38x | 1.12x | 1.05x | 1.77x | 1.20x | 9/10 |
| corpus_uci_sorted | 0.99x | 1.76x | 2.19x | 1.83x | 2.14x | 1.41x | 10/10 |
| replay_trusted_native | 1.01x | 0.89x | 1.91x | 2.14x | 2.78x | 2.98x | 9/10 |
| replay_checked_native | 0.96x | 2.47x | 1.07x | 8.58x | 8.03x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
