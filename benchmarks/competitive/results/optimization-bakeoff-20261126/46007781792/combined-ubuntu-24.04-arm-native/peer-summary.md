| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.42x | 1.69x | 1.83x | 2.20x | 2.19x | 30/30 |
| materialize | 1.01x | 1.98x | 1.55x | 1.36x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.01x | 1.64x | 1.49x | 2.11x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.81x | 1.51x | 2.22x | 2.10x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.30x | 1.25x | 1.72x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.57x | 9.43x | 1.57x | 30.24x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.02x | 1.47x | 1.02x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.39x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.59x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.80x | 130.36x | 175.51x | 140.82x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.99x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.79x | 1.86x | 1.50x | 2.17x | 2.41x | 10/10 |
| corpus_materialize | 1.01x | 1.61x | 1.54x | 1.25x | 1.93x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.66x | 2.02x | 2.26x | 2.29x | 8/10 |
| replay_checked_native | 1.01x | 2.33x | 1.14x | 7.69x | 6.84x | 8.82x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
