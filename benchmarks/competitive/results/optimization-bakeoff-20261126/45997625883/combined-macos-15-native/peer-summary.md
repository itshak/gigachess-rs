| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 5.34x | 1.72x | 1.94x | 2.31x | 2.18x | 29/30 |
| materialize | 0.92x | 1.80x | 1.55x | 1.01x | 2.03x | 1.34x | 24/30 |
| perft_bulk_d4 | 1.00x | 4.42x | 1.52x | 1.22x | 1.97x | 1.83x | 29/30 |
| perft_full_d3 | 1.04x | 1.52x | 1.66x | 1.31x | 1.95x | 2.09x | 30/30 |
| fen_parse | 1.00x | 1.18x | 1.42x | 1.38x | 1.70x | 1.63x | 29/30 |
| fen_write_native_ep | 0.97x | 4.43x | 8.93x | 1.76x | 29.47x | 2.13x | 30/30 |
| snapshot | 1.08x | 1.71x | 1.14x | 2.43x | 1.13x | 1.07x | 27/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 3.71x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 12.58x | N/A | 2.96x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.13x | 1.28x | 5.92x | 1.24x | 1.39x | 28/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 0.69x | 1.27x | 52.75x | 125.91x | 140.52x | 121.04x | 5/5 |
| chess960_materialize | 1.17x | 1.93x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.62x | 1.72x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.20x | 36.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.98x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.99x | 18.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.95x | 2.16x | 1.49x | 2.32x | 2.61x | 10/10 |
| corpus_materialize | 1.17x | 1.89x | 1.73x | 1.28x | 2.25x | 1.42x | 10/10 |
| corpus_uci_sorted | 1.14x | 2.17x | 3.15x | 2.40x | 2.78x | 1.58x | 10/10 |
| replay_trusted_native | 1.01x | 0.65x | 1.77x | 1.63x | 2.02x | 1.76x | 8/10 |
| replay_checked_native | 1.15x | 2.94x | 1.26x | 9.06x | 7.91x | 11.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
