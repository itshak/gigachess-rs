| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 4.24x | 1.54x | 1.49x | 2.00x | 1.82x | 29/30 |
| materialize | 0.92x | 1.97x | 1.46x | 0.97x | 1.98x | 1.38x | 26/30 |
| perft_bulk_d4 | 0.91x | 3.83x | 1.35x | 1.13x | 1.91x | 1.66x | 29/30 |
| perft_full_d3 | 1.02x | 1.58x | 1.76x | 1.35x | 2.01x | 2.02x | 29/30 |
| fen_parse | 1.00x | 1.33x | 1.34x | 1.35x | 1.72x | 1.48x | 30/30 |
| fen_write_native_ep | 0.94x | 4.27x | 8.97x | 1.81x | 26.72x | 2.26x | 30/30 |
| snapshot | 1.05x | 1.67x | 1.13x | 2.15x | 1.26x | 0.89x | 24/30 |
| san_parse_all | 0.95x | 1.75x | N/A | 3.91x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.58x | N/A | 2.50x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.11x | 1.21x | 5.98x | 1.08x | 1.26x | 26/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 0.86x | 1.13x | 64.76x | 149.77x | 143.59x | 132.95x | 5/5 |
| chess960_materialize | 0.85x | 1.71x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.92x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.16x | 44.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.18x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 16.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 5.39x | 1.64x | 1.21x | 1.97x | 2.19x | 10/10 |
| corpus_materialize | 1.19x | 1.44x | 1.38x | 1.10x | 1.66x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.95x | 2.54x | 1.77x | 2.28x | 1.45x | 10/10 |
| replay_trusted_native | 0.85x | 0.57x | 1.59x | 1.67x | 1.58x | 1.57x | 8/10 |
| replay_checked_native | 1.15x | 2.70x | 1.20x | 8.60x | 8.42x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
