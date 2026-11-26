| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.88x | 1.66x | 1.44x | 1.92x | 1.42x | 30/30 |
| materialize | 0.99x | 2.19x | 1.79x | 1.81x | 2.68x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.04x | 1.60x | 1.19x | 1.99x | 1.82x | 29/30 |
| perft_full_d3 | 1.19x | 1.63x | 1.94x | 1.69x | 2.27x | 1.94x | 30/30 |
| fen_parse | 1.09x | 1.53x | 1.45x | 1.23x | 1.49x | 1.20x | 30/30 |
| fen_write_native_ep | 1.02x | 2.95x | 10.32x | 1.84x | 28.94x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.28x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.48x | 4.19x | 1.36x | 1.59x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 56.89x | 110.70x | 121.34x | 103.90x | 5/5 |
| chess960_materialize | 0.95x | 1.72x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 44.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.19x | 1.24x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 21.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.15x | 2.07x | 1.14x | 2.12x | 2.90x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.43x | 1.14x | 1.84x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.96x | 3.02x | 2.18x | 2.99x | 1.54x | 10/10 |
| replay_trusted_native | 0.99x | 0.91x | 1.67x | 1.59x | 2.20x | 2.05x | 9/10 |
| replay_checked_native | 1.01x | 2.11x | 1.17x | 8.93x | 8.76x | 10.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
