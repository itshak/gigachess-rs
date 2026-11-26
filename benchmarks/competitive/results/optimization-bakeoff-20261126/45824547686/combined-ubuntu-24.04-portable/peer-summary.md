| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.73x | 1.55x | 1.35x | 1.83x | 1.30x | 29/30 |
| materialize | 1.06x | 2.14x | 1.74x | 1.74x | 2.66x | 1.90x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.69x | 1.44x | 1.07x | 1.80x | 1.59x | 29/30 |
| perft_full_d3 | 1.00x | 1.51x | 1.75x | 1.56x | 2.14x | 1.76x | 30/30 |
| fen_parse | 0.92x | 1.33x | 1.35x | 1.06x | 1.39x | 1.06x | 30/30 |
| fen_write_native_ep | 0.98x | 2.91x | 9.65x | 1.83x | 26.24x | 2.50x | 30/30 |
| snapshot | 0.99x | 1.75x | 1.24x | 3.24x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 0.99x | 1.88x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.04x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.35x | 1.40x | 3.92x | 1.26x | 1.49x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.44x | 57.03x | 110.05x | 130.09x | 100.48x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.95x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 42.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.74x | 1.89x | 1.03x | 1.93x | 2.56x | 10/10 |
| corpus_materialize | 0.99x | 1.43x | 1.33x | 1.09x | 1.74x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.93x | 2.84x | 2.16x | 2.81x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 0.87x | 1.61x | 1.60x | 2.19x | 2.00x | 9/10 |
| replay_checked_native | 1.02x | 1.84x | 1.03x | 8.17x | 8.28x | 9.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
