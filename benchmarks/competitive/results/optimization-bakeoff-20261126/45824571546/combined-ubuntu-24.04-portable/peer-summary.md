| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.90x | 1.50x | 1.28x | 1.83x | 1.29x | 29/30 |
| materialize | 1.02x | 2.26x | 1.75x | 1.26x | 2.49x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.88x | 1.48x | 1.11x | 1.85x | 1.66x | 29/30 |
| perft_full_d3 | 1.02x | 1.40x | 1.72x | 1.30x | 1.95x | 1.76x | 30/30 |
| fen_parse | 1.01x | 1.64x | 1.61x | 1.22x | 1.97x | 1.27x | 30/30 |
| fen_write_native_ep | 1.02x | 2.81x | 8.48x | 1.52x | 25.06x | 2.09x | 30/30 |
| snapshot | 1.01x | 1.63x | 1.17x | 2.67x | 1.16x | 1.00x | 28/30 |
| san_parse_all | 1.01x | 1.74x | N/A | 3.76x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.93x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.18x | 1.34x | 4.17x | 1.18x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 58.73x | 78.55x | 116.89x | 80.81x | 5/5 |
| chess960_materialize | 1.01x | 1.70x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 31.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.99x | 1.66x | 1.04x | 1.77x | 2.59x | 9/10 |
| corpus_materialize | 1.03x | 1.65x | 1.61x | 1.13x | 2.18x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.04x | 2.84x | 2.26x | 2.81x | 1.58x | 10/10 |
| replay_trusted_native | 0.95x | 0.65x | 1.45x | 1.49x | 1.99x | 1.79x | 8/10 |
| replay_checked_native | 1.02x | 2.08x | 1.19x | 8.54x | 8.23x | 9.70x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
