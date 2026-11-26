| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.95x | 1.63x | 1.63x | 2.37x | 1.95x | 30/30 |
| materialize | 0.97x | 2.18x | 1.61x | 1.20x | 2.35x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.29x | 1.61x | 1.29x | 2.47x | 2.22x | 30/30 |
| perft_full_d3 | 1.31x | 1.85x | 2.16x | 1.63x | 2.82x | 2.68x | 30/30 |
| fen_parse | 0.92x | 1.26x | 1.43x | 1.14x | 1.93x | 1.22x | 29/30 |
| fen_write_native_ep | 0.96x | 3.71x | 8.54x | 1.49x | 24.83x | 1.86x | 30/30 |
| snapshot | 0.99x | 2.49x | 1.50x | 4.49x | 1.49x | 1.50x | 30/30 |
| san_parse_all | 0.98x | 1.76x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.86x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.90x | 1.43x | 4.13x | 1.38x | 1.93x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.25x | 84.94x | 137.69x | 216.01x | 142.45x | 5/5 |
| chess960_materialize | 0.99x | 1.70x | 1.48x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.92x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 34.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 27.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.57x | 1.62x | 1.12x | 2.18x | 2.04x | 9/10 |
| corpus_materialize | 0.98x | 1.66x | 1.54x | 1.13x | 2.21x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.35x | 2.98x | 2.60x | 3.00x | 1.87x | 10/10 |
| replay_trusted_native | 0.98x | 0.74x | 1.38x | 1.64x | 2.49x | 2.87x | 9/10 |
| replay_checked_native | 0.98x | 2.58x | 1.18x | 9.25x | 9.19x | 11.07x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
