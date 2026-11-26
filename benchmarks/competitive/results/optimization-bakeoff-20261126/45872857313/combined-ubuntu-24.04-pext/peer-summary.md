| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.49x | 1.88x | 1.64x | 2.53x | 1.91x | 30/30 |
| materialize | 0.92x | 2.09x | 1.55x | 1.72x | 2.48x | 1.83x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.79x | 1.52x | 1.19x | 2.33x | 1.84x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.54x | 1.50x | 1.95x | 1.68x | 30/30 |
| fen_parse | 1.00x | 1.40x | 1.57x | 1.16x | 1.46x | 1.16x | 30/30 |
| fen_write_native_ep | 1.01x | 4.71x | 10.15x | 1.94x | 27.99x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.16x | 1.34x | 30/30 |
| san_parse_all | 1.02x | 1.91x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.31x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.09x | 1.33x | 3.98x | 1.31x | 1.65x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.09x | 87.16x | 165.18x | 186.62x | 154.36x | 5/5 |
| chess960_materialize | 1.00x | 1.77x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 37.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 19.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.39x | 2.01x | 1.12x | 2.48x | 2.22x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.30x | 1.08x | 1.76x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.28x | 2.95x | 2.25x | 2.85x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.67x | 1.62x | 2.23x | 2.60x | 9/10 |
| replay_checked_native | 0.98x | 2.12x | 1.16x | 8.91x | 8.74x | 10.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
