| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.12x | 1.62x | 1.48x | 1.96x | 1.74x | 30/30 |
| materialize | 0.99x | 1.92x | 1.32x | 1.33x | 2.26x | 1.52x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.72x | 1.53x | 1.40x | 2.04x | 1.85x | 30/30 |
| perft_full_d3 | 0.98x | 1.62x | 1.65x | 1.56x | 2.16x | 1.93x | 30/30 |
| fen_parse | 0.93x | 1.28x | 1.61x | 1.66x | 2.02x | 1.63x | 30/30 |
| fen_write_native_ep | 0.98x | 4.31x | 8.33x | 1.67x | 34.96x | 2.09x | 30/30 |
| snapshot | 0.70x | 1.51x | 1.18x | 2.12x | 1.17x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.97x | N/A | 5.71x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 9.91x | N/A | 3.59x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.74x | 1.03x | 8.56x | 1.08x | 1.62x | 26/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 81.23x | 220.59x | 227.56x | 176.39x | 5/5 |
| chess960_materialize | 1.02x | 1.47x | 1.14x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.39x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 33.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 14.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.97x | 2.39x | 1.45x | 2.79x | 2.76x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.27x | 1.16x | 1.92x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.89x | 2.21x | 1.88x | 2.12x | 1.49x | 10/10 |
| replay_trusted_native | 0.96x | 0.65x | 1.38x | 1.62x | 1.91x | 1.94x | 8/10 |
| replay_checked_native | 0.90x | 1.80x | 0.88x | 6.65x | 6.10x | 7.38x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
