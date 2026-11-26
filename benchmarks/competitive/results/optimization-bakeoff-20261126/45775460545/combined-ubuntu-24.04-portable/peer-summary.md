| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.63x | 1.49x | 1.30x | 1.60x | 1.32x | 29/30 |
| materialize | 0.98x | 2.04x | 1.60x | 1.60x | 2.33x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.97x | 1.56x | 1.17x | 1.82x | 1.76x | 30/30 |
| perft_full_d3 | 1.01x | 0.99x | 1.18x | 1.01x | 1.36x | 1.16x | 22/30 |
| fen_parse | 1.03x | 1.55x | 1.45x | 1.13x | 1.50x | 1.24x | 30/30 |
| fen_write_native_ep | 0.98x | 2.19x | 7.14x | 1.43x | 20.89x | 1.57x | 30/30 |
| snapshot | 1.11x | 1.44x | 1.11x | 2.88x | 1.11x | 1.00x | 27/30 |
| san_parse_all | 1.11x | 1.78x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.52x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.65x | 1.09x | 3.10x | 1.00x | 1.18x | 27/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.06x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.29x | 57.73x | 103.46x | 130.60x | 107.70x | 5/5 |
| chess960_materialize | 1.01x | 1.49x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 1.91x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 39.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 22.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.64x | 2.37x | 1.20x | 2.28x | 3.15x | 10/10 |
| corpus_materialize | 0.99x | 1.53x | 1.43x | 1.15x | 1.83x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.20x | 1.88x | 1.40x | 1.83x | 0.97x | 8/10 |
| replay_trusted_native | 1.07x | 0.63x | 1.25x | 1.18x | 1.60x | 1.48x | 7/10 |
| replay_checked_native | 1.13x | 1.95x | 1.04x | 7.78x | 7.78x | 8.56x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
