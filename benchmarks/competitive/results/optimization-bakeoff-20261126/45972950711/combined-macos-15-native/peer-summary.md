| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 5.18x | 1.74x | 1.86x | 2.44x | 2.07x | 30/30 |
| materialize | 0.97x | 2.13x | 1.62x | 1.08x | 2.41x | 1.42x | 28/30 |
| perft_bulk_d4 | 1.13x | 4.25x | 1.81x | 1.30x | 1.94x | 1.98x | 30/30 |
| perft_full_d3 | 0.88x | 1.52x | 1.60x | 1.21x | 1.76x | 1.74x | 30/30 |
| fen_parse | 1.00x | 1.15x | 1.36x | 1.38x | 2.02x | 1.53x | 29/30 |
| fen_write_native_ep | 1.11x | 4.29x | 13.62x | 1.83x | 29.53x | 2.31x | 30/30 |
| snapshot | 0.95x | 1.87x | 1.25x | 2.43x | 1.08x | 1.18x | 24/30 |
| san_parse_all | 1.12x | 1.89x | N/A | 4.54x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 13.37x | N/A | 2.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.14x | 1.21x | 6.13x | 1.24x | 1.33x | 29/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.59x | 1.95x | 83.95x | 217.58x | 237.85x | 122.61x | 5/5 |
| chess960_materialize | 1.14x | 1.72x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.14x | 3.22x | 1.70x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 40.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.26x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.83x | 21.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.16x | 1.63x | 1.13x | 2.09x | 2.33x | 9/10 |
| corpus_materialize | 1.03x | 1.47x | 1.42x | 1.36x | 1.89x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.79x | 2.33x | 1.65x | 2.38x | 1.29x | 10/10 |
| replay_trusted_native | 1.07x | 0.65x | 1.89x | 2.18x | 1.99x | 2.54x | 8/10 |
| replay_checked_native | 1.02x | 2.50x | 1.11x | 8.81x | 8.72x | 8.92x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
