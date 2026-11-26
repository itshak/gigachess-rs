| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 3.85x | 1.64x | 1.41x | 2.24x | 1.71x | 30/30 |
| materialize | 0.97x | 2.07x | 1.55x | 1.65x | 2.38x | 1.80x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.49x | 1.42x | 1.08x | 2.15x | 1.70x | 28/30 |
| perft_full_d3 | 0.91x | 1.26x | 1.52x | 1.36x | 1.78x | 1.53x | 30/30 |
| fen_parse | 0.93x | 1.20x | 1.38x | 1.02x | 1.42x | 1.18x | 27/30 |
| fen_write_native_ep | 0.92x | 4.49x | 8.96x | 1.95x | 26.76x | 2.45x | 30/30 |
| snapshot | 0.75x | 1.83x | 1.25x | 3.24x | 1.26x | 1.23x | 30/30 |
| san_parse_all | 0.94x | 1.71x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.92x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.04x | 1.34x | 3.93x | 1.29x | 1.62x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.57x | 101.92x | 164.59x | 218.32x | 182.73x | 5/5 |
| chess960_materialize | 0.96x | 1.60x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.27x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 40.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 25.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.56x | 1.79x | 0.93x | 2.13x | 1.94x | 8/10 |
| corpus_materialize | 1.01x | 1.38x | 1.27x | 1.05x | 1.65x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.03x | 2.25x | 2.88x | 2.20x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.54x | 1.51x | 2.12x | 2.50x | 9/10 |
| replay_checked_native | 0.90x | 1.92x | 1.03x | 8.31x | 7.99x | 9.85x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
