| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.24x | 1.46x | 1.47x | 2.31x | 1.73x | 30/30 |
| materialize | 1.00x | 1.91x | 1.62x | 1.20x | 2.22x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.62x | 1.39x | 1.25x | 2.16x | 1.70x | 30/30 |
| perft_full_d3 | 1.02x | 1.27x | 1.42x | 1.26x | 1.84x | 1.69x | 30/30 |
| fen_parse | 1.16x | 1.45x | 1.57x | 1.18x | 2.32x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.09x | 9.62x | 1.72x | 29.01x | 2.05x | 30/30 |
| snapshot | 1.00x | 2.03x | 1.25x | 3.62x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.12x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.46x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.19x | 4.17x | 1.33x | 1.47x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.16x | 136.85x | 232.68x | 148.96x | 5/5 |
| chess960_materialize | 1.02x | 1.50x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.67x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 30.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.06x | 1.64x | 1.26x | 2.44x | 2.12x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.46x | 1.12x | 2.03x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.46x | 3.24x | 2.47x | 3.15x | 1.82x | 10/10 |
| replay_trusted_native | 1.00x | 0.82x | 1.44x | 1.73x | 2.36x | 2.59x | 9/10 |
| replay_checked_native | 1.01x | 2.40x | 1.05x | 8.12x | 7.94x | 9.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
