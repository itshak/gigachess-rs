| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.73x | 1.59x | 1.35x | 1.80x | 1.30x | 29/30 |
| materialize | 0.99x | 2.15x | 1.91x | 1.72x | 2.92x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.72x | 1.45x | 1.07x | 1.79x | 1.60x | 29/30 |
| perft_full_d3 | 0.97x | 1.49x | 1.73x | 1.54x | 2.06x | 1.73x | 30/30 |
| fen_parse | 0.93x | 1.35x | 1.33x | 1.10x | 1.47x | 1.07x | 30/30 |
| fen_write_native_ep | 0.99x | 3.07x | 10.01x | 2.05x | 27.85x | 2.53x | 30/30 |
| snapshot | 1.25x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 0.99x | 1.94x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.20x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.82x | 1.21x | 3.43x | 1.12x | 1.30x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 57.56x | 109.35x | 128.28x | 95.73x | 5/5 |
| chess960_materialize | 1.03x | 1.51x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 1.96x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 43.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 20.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.76x | 1.91x | 1.02x | 1.91x | 2.57x | 10/10 |
| corpus_materialize | 0.95x | 1.44x | 1.35x | 1.11x | 1.77x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.91x | 2.80x | 2.13x | 2.78x | 1.50x | 10/10 |
| replay_trusted_native | 1.02x | 0.71x | 1.33x | 1.33x | 1.81x | 1.66x | 8/10 |
| replay_checked_native | 1.02x | 1.95x | 1.12x | 8.61x | 8.52x | 9.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
