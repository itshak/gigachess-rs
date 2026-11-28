| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.06x | 1.74x | 1.51x | 2.01x | 1.46x | 30/30 |
| materialize | 1.06x | 2.41x | 1.87x | 1.89x | 2.88x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.21x | 1.72x | 1.27x | 2.17x | 1.90x | 29/30 |
| perft_full_d3 | 1.17x | 1.59x | 1.86x | 1.65x | 2.16x | 1.85x | 30/30 |
| fen_parse | 0.93x | 1.61x | 1.30x | 1.09x | 1.38x | 1.13x | 30/30 |
| fen_write_native_ep | 1.00x | 2.95x | 10.03x | 1.81x | 27.98x | 2.34x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.41x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.26x | 1.45x | 4.25x | 1.40x | 1.57x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 61.42x | 107.13x | 125.19x | 108.05x | 5/5 |
| chess960_materialize | 0.93x | 1.74x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.14x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 42.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.61x | 1.60x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.04x | 25.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.87x | 2.46x | 1.34x | 2.50x | 3.34x | 10/10 |
| corpus_materialize | 1.01x | 1.51x | 1.41x | 1.16x | 1.87x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.94x | 3.00x | 2.24x | 2.93x | 1.53x | 10/10 |
| replay_trusted_native | 0.95x | 0.86x | 1.62x | 1.62x | 2.16x | 2.00x | 9/10 |
| replay_checked_native | 1.00x | 2.13x | 1.18x | 8.99x | 8.82x | 10.00x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
