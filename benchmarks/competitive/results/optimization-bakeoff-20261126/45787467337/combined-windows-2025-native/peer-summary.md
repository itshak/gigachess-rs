| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.34x | 1.50x | 1.46x | 2.12x | 1.87x | 30/30 |
| materialize | 1.00x | 1.92x | 1.30x | 1.18x | 2.18x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.84x | 1.51x | 1.35x | 2.19x | 1.93x | 30/30 |
| perft_full_d3 | 0.98x | 1.30x | 1.44x | 1.23x | 1.82x | 1.75x | 30/30 |
| fen_parse | 1.02x | 1.43x | 1.50x | 1.40x | 2.32x | 1.48x | 30/30 |
| fen_write_native_ep | 1.03x | 4.23x | 8.73x | 1.65x | 33.32x | 2.01x | 30/30 |
| snapshot | 1.30x | 1.55x | 1.18x | 2.44x | 1.19x | 1.02x | 30/30 |
| san_parse_all | 1.06x | 2.10x | N/A | 5.68x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.91x | N/A | 3.73x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.93x | 1.19x | 8.13x | 1.26x | 1.42x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 68.13x | 140.41x | 200.62x | 150.74x | 5/5 |
| chess960_materialize | 0.99x | 1.53x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.56x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 31.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 20.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.34x | 1.77x | 1.33x | 2.47x | 2.40x | 10/10 |
| corpus_materialize | 0.98x | 1.61x | 1.29x | 1.15x | 1.97x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.77x | 2.25x | 1.92x | 2.17x | 1.41x | 10/10 |
| replay_trusted_native | 1.02x | 0.69x | 1.32x | 1.76x | 2.20x | 2.55x | 8/10 |
| replay_checked_native | 1.02x | 2.24x | 1.02x | 7.45x | 7.59x | 9.11x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
