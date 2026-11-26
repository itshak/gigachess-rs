| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.47x | 1.58x | 1.90x | 1.90x | 30/30 |
| materialize | 1.00x | 1.83x | 1.44x | 1.26x | 2.00x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.74x | 1.53x | 1.39x | 1.94x | 2.00x | 30/30 |
| perft_full_d3 | 1.00x | 1.15x | 1.25x | 1.05x | 1.50x | 1.47x | 29/30 |
| fen_parse | 0.99x | 1.25x | 1.33x | 1.27x | 1.78x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.23x | 1.57x | 29.51x | 2.39x | 30/30 |
| snapshot | 0.96x | 1.13x | 1.06x | 1.52x | 1.07x | 0.96x | 23/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.84x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.13x | 1.34x | 3.62x | 1.09x | 1.08x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.17x | 69.05x | 116.66x | 155.52x | 126.62x | 5/5 |
| chess960_materialize | 1.00x | 1.53x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.62x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 26.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.78x | 15.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.80x | 1.84x | 1.46x | 2.19x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.49x | 1.21x | 1.90x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.92x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.92x | 0.48x | 1.25x | 1.51x | 1.69x | 1.75x | 8/10 |
| replay_checked_native | 0.99x | 1.98x | 0.97x | 6.54x | 5.81x | 7.48x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
