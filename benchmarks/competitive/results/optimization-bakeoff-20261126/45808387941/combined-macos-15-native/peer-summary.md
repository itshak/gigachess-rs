| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.57x | 1.59x | 1.51x | 2.00x | 1.73x | 30/30 |
| materialize | 1.07x | 1.99x | 1.52x | 0.93x | 2.30x | 1.31x | 25/30 |
| perft_bulk_d4 | 0.86x | 3.38x | 1.27x | 1.11x | 1.67x | 1.72x | 28/30 |
| perft_full_d3 | 0.93x | 1.54x | 1.74x | 1.27x | 1.85x | 1.92x | 29/30 |
| fen_parse | 0.96x | 1.27x | 1.30x | 1.28x | 1.66x | 1.44x | 30/30 |
| fen_write_native_ep | 1.04x | 4.34x | 9.21x | 1.81x | 28.27x | 2.32x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.11x | 2.19x | 1.11x | 0.92x | 24/30 |
| san_parse_all | 0.91x | 1.76x | N/A | 3.62x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 12.15x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.10x | 1.09x | 1.16x | 5.40x | 1.13x | 1.26x | 28/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.18x | 71.55x | 148.20x | 156.84x | 129.03x | 5/5 |
| chess960_materialize | 1.26x | 1.56x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.86x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 39.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 19.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.75x | 4.97x | 1.65x | 1.09x | 2.00x | 2.17x | 9/10 |
| corpus_materialize | 0.96x | 1.38x | 1.34x | 1.11x | 1.83x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.78x | 2.51x | 1.99x | 2.42x | 1.34x | 10/10 |
| replay_trusted_native | 1.11x | 0.59x | 1.46x | 1.49x | 1.55x | 1.57x | 8/10 |
| replay_checked_native | 1.19x | 2.13x | 1.05x | 7.70x | 7.17x | 8.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
