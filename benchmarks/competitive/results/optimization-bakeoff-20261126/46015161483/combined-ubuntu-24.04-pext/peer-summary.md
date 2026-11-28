| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.86x | 1.94x | 1.69x | 2.62x | 1.98x | 30/30 |
| materialize | 1.03x | 2.28x | 1.76x | 1.79x | 2.72x | 1.86x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.03x | 1.59x | 1.26x | 2.46x | 1.95x | 29/30 |
| perft_full_d3 | 1.19x | 1.64x | 1.91x | 1.84x | 2.38x | 2.02x | 30/30 |
| fen_parse | 0.99x | 1.49x | 1.39x | 1.17x | 1.59x | 1.19x | 30/30 |
| fen_write_native_ep | 0.97x | 4.54x | 10.44x | 1.97x | 31.07x | 2.59x | 30/30 |
| snapshot | 0.95x | 1.84x | 1.17x | 4.40x | 2.34x | 1.36x | 30/30 |
| san_parse_all | 0.96x | 1.70x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.23x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.13x | 1.36x | 4.07x | 1.34x | 1.66x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.08x | 1.12x | 87.95x | 161.13x | 208.67x | 168.37x | 5/5 |
| chess960_materialize | 1.08x | 1.72x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.57x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 38.15x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.55x | 1.60x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.90x | 18.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.18x | 6.06x | 2.26x | 1.24x | 2.77x | 2.54x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.10x | 1.74x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.95x | 2.29x | 3.00x | 2.15x | 2.90x | 1.51x | 10/10 |
| replay_trusted_native | 1.03x | 0.90x | 1.55x | 1.61x | 2.23x | 2.59x | 9/10 |
| replay_checked_native | 1.02x | 2.18x | 1.10x | 9.00x | 8.68x | 10.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
