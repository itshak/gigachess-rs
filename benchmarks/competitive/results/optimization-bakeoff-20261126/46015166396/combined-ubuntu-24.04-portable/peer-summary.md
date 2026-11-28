| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.05x | 1.74x | 1.52x | 2.07x | 1.46x | 30/30 |
| materialize | 1.00x | 2.41x | 1.95x | 1.83x | 2.95x | 2.02x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.21x | 1.71x | 1.27x | 2.13x | 1.91x | 30/30 |
| perft_full_d3 | 1.16x | 1.60x | 1.87x | 1.65x | 2.26x | 1.86x | 30/30 |
| fen_parse | 0.92x | 1.59x | 1.29x | 1.09x | 1.37x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 3.08x | 10.10x | 1.80x | 28.11x | 2.33x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.06x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.02x | 1.43x | 4.07x | 1.29x | 1.49x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 60.69x | 108.46x | 122.87x | 123.91x | 5/5 |
| chess960_materialize | 0.98x | 1.82x | 1.77x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.14x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 39.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.62x | 1.61x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 24.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 4.87x | 2.47x | 1.34x | 2.47x | 3.34x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.42x | 1.15x | 1.87x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.94x | 3.01x | 2.22x | 2.93x | 1.53x | 10/10 |
| replay_trusted_native | 0.96x | 0.86x | 1.64x | 1.60x | 2.15x | 2.00x | 9/10 |
| replay_checked_native | 1.00x | 2.14x | 1.19x | 9.06x | 8.82x | 10.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
