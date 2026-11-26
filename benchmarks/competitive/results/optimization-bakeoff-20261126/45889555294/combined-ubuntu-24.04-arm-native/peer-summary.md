| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 3.94x | 1.50x | 1.62x | 1.96x | 1.95x | 30/30 |
| materialize | 0.98x | 1.89x | 1.48x | 1.30x | 2.09x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.93x | 3.57x | 1.46x | 1.32x | 1.87x | 1.91x | 30/30 |
| perft_full_d3 | 0.83x | 1.22x | 1.33x | 1.15x | 1.62x | 1.53x | 30/30 |
| fen_parse | 0.97x | 1.21x | 1.30x | 1.24x | 1.70x | 1.36x | 30/30 |
| fen_write_native_ep | 0.95x | 4.37x | 9.00x | 1.52x | 28.86x | 2.27x | 30/30 |
| snapshot | 1.39x | 1.59x | 1.18x | 2.01x | 1.14x | 1.28x | 30/30 |
| san_parse_all | 0.95x | 1.77x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 11.86x | N/A | 2.59x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.19x | 1.38x | 3.90x | 1.14x | 1.16x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.45x | N/A | N/A | 6/6 |
| startpos | 1.09x | 1.29x | 82.97x | 143.11x | 184.74x | 147.46x | 5/5 |
| chess960_materialize | 0.96x | 1.63x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.73x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 26.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.13x | 21.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 4.06x | 1.58x | 1.26x | 1.84x | 2.05x | 10/10 |
| corpus_materialize | 0.97x | 1.55x | 1.49x | 1.19x | 1.85x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.15x | 3.00x | 2.12x | 2.91x | 1.49x | 10/10 |
| replay_trusted_native | 0.95x | 0.60x | 1.58x | 1.89x | 2.08x | 2.15x | 8/10 |
| replay_checked_native | 0.91x | 2.10x | 1.02x | 6.83x | 6.11x | 7.97x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
