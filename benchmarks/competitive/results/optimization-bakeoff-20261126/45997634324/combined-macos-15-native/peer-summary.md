| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.15x | 5.16x | 1.71x | 1.84x | 2.25x | 2.23x | 30/30 |
| materialize | 1.06x | 1.87x | 1.46x | 1.14x | 2.18x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.12x | 4.37x | 1.69x | 1.27x | 2.04x | 1.99x | 29/30 |
| perft_full_d3 | 1.19x | 1.81x | 1.95x | 1.42x | 2.30x | 2.04x | 30/30 |
| fen_parse | 1.10x | 1.29x | 1.36x | 1.27x | 1.72x | 1.53x | 30/30 |
| fen_write_native_ep | 1.03x | 4.52x | 9.22x | 1.72x | 27.15x | 2.24x | 30/30 |
| snapshot | 1.10x | 1.61x | 1.14x | 2.65x | 1.27x | 1.17x | 27/30 |
| san_parse_all | 1.08x | 1.76x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.90x | N/A | 2.81x | N/A | N/A | 12/12 |
| apply_children_full | 1.15x | 1.23x | 1.27x | 5.84x | 1.20x | 1.36x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.30x | 1.05x | 74.97x | 130.92x | 156.41x | 124.40x | 5/5 |
| chess960_materialize | 1.15x | 1.77x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.26x | 3.33x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 38.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 17.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.24x | 6.60x | 2.14x | 1.27x | 2.85x | 2.78x | 10/10 |
| corpus_materialize | 1.02x | 1.63x | 1.37x | 1.12x | 1.78x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.12x | 1.92x | 2.59x | 1.82x | 2.65x | 1.36x | 10/10 |
| replay_trusted_native | 1.12x | 0.71x | 1.75x | 1.95x | 2.09x | 1.97x | 8/10 |
| replay_checked_native | 1.11x | 2.55x | 1.22x | 9.45x | 7.98x | 9.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
