| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.52x | 1.55x | 1.62x | 2.10x | 1.70x | 30/30 |
| materialize | 0.88x | 1.95x | 1.55x | 1.10x | 2.35x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.76x | 3.55x | 1.37x | 1.05x | 1.66x | 1.60x | 26/30 |
| perft_full_d3 | 0.91x | 1.44x | 1.52x | 1.13x | 1.71x | 1.82x | 29/30 |
| fen_parse | 0.91x | 1.29x | 1.30x | 1.28x | 1.55x | 1.47x | 30/30 |
| fen_write_native_ep | 0.98x | 4.43x | 9.86x | 1.83x | 29.47x | 2.29x | 30/30 |
| snapshot | 0.97x | 1.62x | 1.36x | 2.40x | 1.20x | 1.08x | 27/30 |
| san_parse_all | 0.81x | 1.83x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 12.23x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.02x | 1.05x | 5.24x | 1.04x | 1.18x | 23/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.21x | N/A | N/A | 5/6 |
| startpos | 0.98x | 1.20x | 75.97x | 177.19x | 208.83x | 150.28x | 5/5 |
| chess960_materialize | 0.91x | 1.55x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.74x | 2.51x | 1.17x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 37.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.97x | 19.65x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 5.07x | 1.66x | 0.97x | 1.92x | 2.13x | 8/10 |
| corpus_materialize | 0.94x | 1.36x | 1.32x | 1.15x | 1.99x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.06x | 2.55x | 2.03x | 2.74x | 1.45x | 10/10 |
| replay_trusted_native | 1.13x | 0.60x | 1.60x | 1.56x | 1.71x | 1.69x | 8/10 |
| replay_checked_native | 0.73x | 1.77x | 0.91x | 6.95x | 5.97x | 8.40x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
