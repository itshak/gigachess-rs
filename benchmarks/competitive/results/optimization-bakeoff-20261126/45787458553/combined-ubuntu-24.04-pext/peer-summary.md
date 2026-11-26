| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.75x | 1.63x | 1.64x | 2.57x | 1.89x | 30/30 |
| materialize | 0.96x | 2.08x | 1.73x | 1.23x | 2.14x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.97x | 1.53x | 1.26x | 2.48x | 1.99x | 30/30 |
| perft_full_d3 | 0.92x | 1.12x | 1.15x | 0.88x | 1.66x | 1.32x | 24/30 |
| fen_parse | 0.90x | 1.27x | 1.33x | 1.06x | 1.70x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 4.26x | 9.73x | 1.84x | 27.46x | 2.33x | 30/30 |
| snapshot | 1.23x | 2.11x | 1.25x | 3.81x | 1.30x | 1.25x | 30/30 |
| san_parse_all | 0.94x | 2.06x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.04x | N/A | 2.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.48x | 0.97x | 3.49x | 1.13x | 1.27x | 26/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.08x | N/A | N/A | 5/6 |
| startpos | 0.91x | 1.00x | 84.47x | 125.67x | 177.70x | 135.00x | 5/5 |
| chess960_materialize | 1.01x | 1.80x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.83x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 17.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.88x | 1.87x | 1.43x | 2.60x | 2.11x | 10/10 |
| corpus_materialize | 1.02x | 2.02x | 1.83x | 1.30x | 2.93x | 1.67x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.43x | 3.25x | 2.38x | 3.05x | 1.69x | 10/10 |
| replay_trusted_native | 1.05x | 0.81x | 1.35x | 1.55x | 2.77x | 3.35x | 8/10 |
| replay_checked_native | 0.94x | 2.33x | 0.92x | 5.11x | 5.79x | 6.97x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
