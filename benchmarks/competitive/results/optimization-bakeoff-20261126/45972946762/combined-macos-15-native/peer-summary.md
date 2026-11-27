| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.62x | 1.68x | 1.51x | 2.42x | 1.95x | 30/30 |
| materialize | 0.96x | 2.19x | 1.60x | 1.19x | 2.16x | 1.54x | 29/30 |
| perft_bulk_d4 | 1.11x | 4.21x | 1.70x | 1.33x | 2.00x | 1.93x | 30/30 |
| perft_full_d3 | 1.02x | 1.74x | 1.90x | 1.36x | 2.07x | 2.05x | 30/30 |
| fen_parse | 0.99x | 1.26x | 1.47x | 1.28x | 1.96x | 1.43x | 29/30 |
| fen_write_native_ep | 0.94x | 4.26x | 13.70x | 1.72x | 28.77x | 2.16x | 30/30 |
| snapshot | 1.22x | 2.09x | 1.28x | 2.54x | 1.18x | 1.25x | 25/30 |
| san_parse_all | 1.00x | 1.81x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 12.45x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.16x | 1.24x | 5.58x | 1.19x | 1.33x | 28/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.22x | 1.99x | 101.74x | 183.15x | 262.06x | 179.65x | 5/5 |
| chess960_materialize | 1.10x | 1.74x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 3.54x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 36.04x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.79x | 1.16x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 19.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 6.23x | 1.90x | 1.42x | 2.38x | 2.45x | 10/10 |
| corpus_materialize | 0.87x | 1.56x | 1.37x | 1.12x | 1.89x | 1.31x | 10/10 |
| corpus_uci_sorted | 0.93x | 1.97x | 2.73x | 1.93x | 2.74x | 1.34x | 10/10 |
| replay_trusted_native | 0.89x | 0.75x | 1.85x | 1.97x | 2.27x | 1.92x | 8/10 |
| replay_checked_native | 0.89x | 2.53x | 1.01x | 8.44x | 7.27x | 10.05x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
