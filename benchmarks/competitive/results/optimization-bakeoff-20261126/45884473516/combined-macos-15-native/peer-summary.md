| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 5.06x | 1.64x | 1.74x | 2.60x | 2.17x | 30/30 |
| materialize | 0.83x | 1.78x | 1.37x | 0.96x | 2.05x | 1.24x | 24/30 |
| perft_bulk_d4 | 0.93x | 4.20x | 1.57x | 1.21x | 1.96x | 1.85x | 29/30 |
| perft_full_d3 | 0.97x | 1.59x | 1.78x | 1.38x | 2.10x | 2.14x | 30/30 |
| fen_parse | 1.04x | 1.22x | 1.46x | 1.42x | 1.69x | 1.47x | 30/30 |
| fen_write_native_ep | 0.95x | 3.93x | 8.56x | 1.66x | 26.16x | 2.16x | 30/30 |
| snapshot | 0.94x | 1.72x | 1.23x | 2.57x | 1.02x | 1.03x | 21/30 |
| san_parse_all | 0.96x | 1.79x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 13.00x | N/A | 3.03x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.19x | 1.32x | 6.75x | 1.21x | 1.51x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.29x | N/A | N/A | 5/6 |
| startpos | 1.47x | 1.25x | 74.00x | 152.11x | 173.76x | 167.31x | 5/5 |
| chess960_materialize | 1.03x | 1.86x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 3.58x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.11x | 47.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 19.96x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.48x | 2.22x | 1.43x | 2.37x | 2.58x | 10/10 |
| corpus_materialize | 0.99x | 1.66x | 1.51x | 1.28x | 2.02x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.20x | 2.09x | 2.81x | 2.13x | 2.64x | 1.59x | 10/10 |
| replay_trusted_native | 0.83x | 0.53x | 1.31x | 1.52x | 1.44x | 1.52x | 8/10 |
| replay_checked_native | 1.09x | 2.30x | 1.22x | 8.34x | 6.96x | 9.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
