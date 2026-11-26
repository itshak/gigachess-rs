| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 5.20x | 1.70x | 1.76x | 2.48x | 2.13x | 30/30 |
| materialize | 0.96x | 2.06x | 1.54x | 1.11x | 2.30x | 1.40x | 28/30 |
| perft_bulk_d4 | 0.97x | 3.99x | 1.46x | 1.22x | 1.84x | 1.84x | 30/30 |
| perft_full_d3 | 1.02x | 1.56x | 1.71x | 1.35x | 1.98x | 2.02x | 30/30 |
| fen_parse | 0.97x | 1.23x | 1.32x | 1.35x | 1.60x | 1.72x | 30/30 |
| fen_write_native_ep | 1.09x | 4.26x | 9.53x | 1.91x | 30.47x | 2.43x | 30/30 |
| snapshot | 1.02x | 1.78x | 1.22x | 2.60x | 1.15x | 1.01x | 26/30 |
| san_parse_all | 1.01x | 1.75x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.16x | N/A | 2.73x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.15x | 1.26x | 5.86x | 1.17x | 1.37x | 29/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.20x | 69.60x | 162.20x | 167.28x | 167.13x | 5/5 |
| chess960_materialize | 0.97x | 1.77x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 3.28x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 37.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.19x | 1.17x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 18.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 6.40x | 1.93x | 1.18x | 2.25x | 2.43x | 9/10 |
| corpus_materialize | 1.10x | 1.52x | 1.46x | 1.15x | 1.96x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.81x | 2.56x | 1.82x | 2.36x | 1.32x | 10/10 |
| replay_trusted_native | 1.03x | 0.66x | 1.67x | 1.74x | 1.86x | 1.89x | 8/10 |
| replay_checked_native | 1.05x | 2.64x | 1.22x | 9.24x | 8.09x | 9.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
