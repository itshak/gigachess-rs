| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.33x | 1.57x | 1.51x | 2.40x | 1.91x | 30/30 |
| materialize | 1.00x | 1.94x | 1.29x | 1.08x | 2.20x | 1.47x | 29/30 |
| perft_bulk_d4 | 0.99x | 3.75x | 1.53x | 1.35x | 2.46x | 2.09x | 30/30 |
| perft_full_d3 | 0.95x | 1.19x | 1.25x | 1.02x | 1.69x | 1.47x | 26/30 |
| fen_parse | 1.01x | 1.25x | 1.43x | 1.30x | 1.89x | 1.30x | 30/30 |
| fen_write_native_ep | 1.01x | 4.02x | 8.03x | 1.59x | 31.22x | 2.00x | 30/30 |
| snapshot | 0.72x | 1.19x | 0.85x | 1.92x | 0.85x | 0.78x | 12/30 |
| san_parse_all | 1.01x | 2.02x | N/A | 4.70x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 8.42x | N/A | 3.00x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.51x | 1.04x | 6.13x | 1.16x | 1.73x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.18x | 93.86x | 152.81x | 219.64x | 158.16x | 5/5 |
| chess960_materialize | 1.01x | 1.52x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.73x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 16.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.45x | 1.80x | 1.31x | 2.38x | 2.17x | 10/10 |
| corpus_materialize | 1.00x | 1.76x | 1.37x | 1.15x | 2.78x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.87x | 2.37x | 1.95x | 2.27x | 1.50x | 10/10 |
| replay_trusted_native | 1.16x | 0.76x | 1.32x | 1.62x | 2.60x | 3.27x | 8/10 |
| replay_checked_native | 1.04x | 2.15x | 0.97x | 5.48x | 6.74x | 7.48x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
