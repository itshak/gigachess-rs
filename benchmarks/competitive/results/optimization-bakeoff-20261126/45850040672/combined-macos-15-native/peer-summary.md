| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.07x | 1.68x | 1.74x | 2.69x | 2.12x | 30/30 |
| materialize | 1.11x | 2.16x | 1.66x | 1.10x | 2.27x | 1.58x | 28/30 |
| perft_bulk_d4 | 1.03x | 4.32x | 1.51x | 1.25x | 1.74x | 1.81x | 29/30 |
| perft_full_d3 | 1.05x | 1.60x | 1.88x | 1.45x | 2.13x | 2.33x | 29/30 |
| fen_parse | 1.00x | 1.30x | 1.35x | 1.35x | 1.62x | 1.47x | 30/30 |
| fen_write_native_ep | 1.13x | 3.95x | 9.02x | 1.85x | 28.60x | 2.25x | 30/30 |
| snapshot | 0.99x | 1.96x | 1.56x | 2.46x | 1.07x | 1.02x | 25/30 |
| san_parse_all | 1.09x | 1.99x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 1.19x | 13.24x | N/A | 3.03x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.15x | 1.26x | 6.11x | 1.17x | 1.46x | 26/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.76x | 1.33x | 80.66x | 157.15x | 197.56x | 222.59x | 5/5 |
| chess960_materialize | 1.02x | 1.76x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.28x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 37.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.07x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.13x | 6.08x | 1.92x | 1.28x | 2.32x | 2.61x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.34x | 1.12x | 1.69x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.92x | 1.53x | 2.35x | 1.60x | 1.87x | 1.09x | 9/10 |
| replay_trusted_native | 1.19x | 0.71x | 1.90x | 1.95x | 2.04x | 2.07x | 8/10 |
| replay_checked_native | 1.19x | 2.64x | 1.20x | 8.46x | 7.89x | 10.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
