| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 5.06x | 1.61x | 1.76x | 2.53x | 2.27x | 30/30 |
| materialize | 0.93x | 2.16x | 1.59x | 1.07x | 2.22x | 1.47x | 27/30 |
| perft_bulk_d4 | 0.91x | 4.01x | 1.52x | 1.27x | 1.89x | 1.75x | 29/30 |
| perft_full_d3 | 0.90x | 1.46x | 1.65x | 1.31x | 1.82x | 1.94x | 29/30 |
| fen_parse | 0.90x | 1.13x | 1.27x | 1.26x | 1.74x | 1.30x | 28/30 |
| fen_write_native_ep | 0.97x | 4.13x | 13.52x | 1.69x | 27.01x | 2.19x | 30/30 |
| snapshot | 0.91x | 1.55x | 1.09x | 2.23x | 1.01x | 0.92x | 20/30 |
| san_parse_all | 0.98x | 1.97x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 11.56x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.16x | 1.35x | 5.86x | 1.28x | 1.38x | 28/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.32x | N/A | N/A | 5/6 |
| startpos | 1.20x | 1.24x | 84.30x | 154.45x | 256.52x | 184.30x | 5/5 |
| chess960_materialize | 0.97x | 1.98x | 2.09x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 3.24x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 45.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 20.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 5.51x | 1.76x | 1.22x | 1.98x | 2.41x | 9/10 |
| corpus_materialize | 0.81x | 1.38x | 1.29x | 0.99x | 1.63x | 1.12x | 9/10 |
| corpus_uci_sorted | 0.93x | 1.78x | 2.47x | 1.79x | 2.36x | 1.42x | 10/10 |
| replay_trusted_native | 1.09x | 0.73x | 2.11x | 1.91x | 1.90x | 2.07x | 8/10 |
| replay_checked_native | 1.00x | 2.60x | 1.09x | 8.84x | 7.85x | 9.43x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
