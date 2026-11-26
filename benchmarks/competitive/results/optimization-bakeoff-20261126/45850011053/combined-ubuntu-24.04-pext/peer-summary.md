| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.88x | 1.64x | 1.62x | 2.38x | 2.02x | 30/30 |
| materialize | 0.97x | 2.13x | 1.64x | 1.25x | 2.39x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.23x | 1.59x | 1.28x | 2.43x | 2.22x | 30/30 |
| perft_full_d3 | 0.88x | 1.25x | 1.49x | 1.11x | 1.91x | 1.82x | 30/30 |
| fen_parse | 0.85x | 1.22x | 1.39x | 1.08x | 1.81x | 1.16x | 28/30 |
| fen_write_native_ep | 0.99x | 4.07x | 8.72x | 1.46x | 25.24x | 1.92x | 30/30 |
| snapshot | 0.99x | 2.49x | 1.50x | 4.49x | 1.50x | 1.50x | 30/30 |
| san_parse_all | 0.93x | 1.69x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 9.56x | N/A | 2.02x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.85x | 1.33x | 4.02x | 1.33x | 1.83x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.73x | 0.91x | 61.25x | 101.88x | 162.16x | 107.90x | 4/5 |
| chess960_materialize | 0.99x | 1.77x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.68x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 38.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.21x | 28.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.96x | 1.73x | 1.17x | 2.27x | 2.11x | 9/10 |
| corpus_materialize | 1.05x | 1.71x | 1.60x | 1.18x | 2.27x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.45x | 3.02x | 2.44x | 3.12x | 1.71x | 10/10 |
| replay_trusted_native | 1.04x | 0.72x | 1.36x | 1.50x | 2.37x | 2.77x | 8/10 |
| replay_checked_native | 0.92x | 2.25x | 1.02x | 7.96x | 7.89x | 10.62x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
