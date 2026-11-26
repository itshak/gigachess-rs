| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.73x | 1.53x | 1.33x | 1.63x | 1.43x | 30/30 |
| materialize | 0.99x | 2.11x | 1.69x | 1.65x | 2.43x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.03x | 1.59x | 1.18x | 1.84x | 1.81x | 30/30 |
| perft_full_d3 | 1.02x | 1.63x | 1.88x | 1.65x | 2.30x | 1.92x | 30/30 |
| fen_parse | 1.01x | 1.34x | 1.52x | 1.29x | 1.54x | 1.27x | 30/30 |
| fen_write_native_ep | 1.00x | 3.06x | 9.74x | 1.96x | 28.56x | 2.68x | 30/30 |
| snapshot | 0.82x | 1.21x | 1.02x | 2.84x | 1.00x | 1.00x | 28/30 |
| san_parse_all | 0.98x | 1.77x | N/A | 4.13x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.77x | N/A | 2.25x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.80x | 1.17x | 3.36x | 1.07x | 1.29x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.60x | 112.78x | 125.20x | 104.42x | 5/5 |
| chess960_materialize | 1.00x | 1.54x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.92x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 40.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.91x | 23.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.75x | 2.43x | 1.22x | 2.35x | 3.26x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.45x | 1.15x | 1.87x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.99x | 2.96x | 2.24x | 2.91x | 1.58x | 10/10 |
| replay_trusted_native | 0.99x | 0.68x | 1.28x | 1.32x | 1.76x | 1.62x | 8/10 |
| replay_checked_native | 1.04x | 2.19x | 1.09x | 8.64x | 8.58x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
