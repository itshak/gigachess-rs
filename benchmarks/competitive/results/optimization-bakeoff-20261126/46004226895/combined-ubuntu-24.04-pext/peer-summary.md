| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.75x | 1.96x | 1.71x | 2.67x | 2.02x | 30/30 |
| materialize | 1.01x | 2.31x | 1.69x | 1.76x | 2.73x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.03x | 1.60x | 1.25x | 2.49x | 1.97x | 29/30 |
| perft_full_d3 | 1.24x | 1.67x | 2.01x | 1.90x | 2.51x | 2.04x | 30/30 |
| fen_parse | 0.99x | 1.34x | 1.36x | 1.06x | 1.46x | 1.24x | 30/30 |
| fen_write_native_ep | 1.01x | 4.60x | 10.46x | 2.15x | 28.47x | 2.64x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.68x | 1.00x | 1.33x | 26/30 |
| san_parse_all | 1.01x | 1.89x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.09x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.13x | 1.34x | 4.00x | 1.42x | 1.63x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 96.31x | 137.55x | 168.31x | 151.34x | 4/5 |
| chess960_materialize | 0.93x | 1.71x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.64x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 38.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.80x | 19.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.24x | 2.32x | 1.28x | 2.80x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.29x | 1.08x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.30x | 2.99x | 2.24x | 2.88x | 1.58x | 10/10 |
| replay_trusted_native | 1.01x | 0.91x | 1.65x | 1.58x | 2.26x | 2.59x | 9/10 |
| replay_checked_native | 0.98x | 2.13x | 1.18x | 8.96x | 8.60x | 10.27x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
