| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.94x | 1.67x | 1.42x | 2.18x | 1.71x | 30/30 |
| materialize | 0.99x | 1.97x | 1.56x | 1.58x | 2.36x | 1.78x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.29x | 1.35x | 1.02x | 2.00x | 1.61x | 28/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.66x | 1.57x | 2.00x | 1.79x | 30/30 |
| fen_parse | 1.07x | 1.38x | 1.63x | 1.09x | 1.52x | 1.17x | 30/30 |
| fen_write_native_ep | 0.96x | 4.38x | 9.61x | 2.06x | 27.16x | 2.69x | 30/30 |
| snapshot | 2.36x | 1.41x | 1.15x | 3.21x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 0.99x | 1.93x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.99x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.73x | 1.11x | 3.21x | 1.11x | 1.33x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.21x | 81.62x | 139.54x | 182.18x | 148.59x | 5/5 |
| chess960_materialize | 0.99x | 1.54x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.33x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 39.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 21.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.23x | 1.59x | 0.87x | 1.93x | 1.74x | 8/10 |
| corpus_materialize | 0.95x | 1.35x | 1.26x | 1.04x | 1.67x | 1.15x | 9/10 |
| corpus_uci_sorted | 1.01x | 2.24x | 2.94x | 2.19x | 2.82x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.73x | 1.30x | 1.27x | 1.82x | 2.11x | 7/10 |
| replay_checked_native | 0.99x | 2.01x | 1.12x | 8.46x | 8.26x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
