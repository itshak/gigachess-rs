| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.47x | 1.58x | 1.90x | 1.90x | 30/30 |
| materialize | 0.99x | 1.80x | 1.41x | 1.24x | 1.99x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.71x | 1.52x | 1.38x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 0.99x | 1.18x | 1.27x | 1.08x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.27x | 1.77x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 3.01x | 6.25x | 1.07x | 19.90x | 1.59x | 29/30 |
| snapshot | 0.99x | 1.22x | 0.95x | 1.56x | 0.90x | 1.02x | 18/30 |
| san_parse_all | 1.00x | 1.86x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 1.28x | 1.47x | 4.13x | 1.22x | 1.23x | 30/30 |
| make_unmake_full | 1.13x | N/A | N/A | 1.62x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 76.40x | 129.43x | 170.25x | 137.24x | 5/5 |
| chess960_materialize | 1.00x | 1.50x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.59x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 26.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.79x | 1.83x | 1.45x | 2.16x | 2.47x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.48x | 1.19x | 1.85x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.09x | 2.93x | 2.08x | 2.78x | 1.45x | 10/10 |
| replay_trusted_native | 1.22x | 0.62x | 1.63x | 1.95x | 2.18x | 2.23x | 8/10 |
| replay_checked_native | 0.97x | 1.98x | 0.96x | 6.43x | 5.75x | 7.60x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
