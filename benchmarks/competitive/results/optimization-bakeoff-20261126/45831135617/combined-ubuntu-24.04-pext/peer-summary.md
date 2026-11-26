| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 3.81x | 1.60x | 1.39x | 2.14x | 1.62x | 30/30 |
| materialize | 0.97x | 2.07x | 1.53x | 1.61x | 2.48x | 1.79x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.24x | 1.30x | 1.02x | 2.04x | 1.59x | 28/30 |
| perft_full_d3 | 0.95x | 1.36x | 1.54x | 1.52x | 2.15x | 1.63x | 30/30 |
| fen_parse | 1.06x | 1.37x | 1.35x | 1.15x | 1.56x | 1.22x | 30/30 |
| fen_write_native_ep | 0.95x | 4.62x | 9.78x | 1.77x | 27.83x | 2.36x | 30/30 |
| snapshot | 0.60x | 1.20x | 1.00x | 2.60x | 1.00x | 1.00x | 22/30 |
| san_parse_all | 1.04x | 1.93x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.13x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.31x | 3.92x | 1.31x | 1.53x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.10x | N/A | N/A | 4/6 |
| startpos | 1.49x | 2.33x | 132.00x | 253.57x | 309.66x | 258.80x | 5/5 |
| chess960_materialize | 0.85x | 1.54x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.29x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 40.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 24.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.35x | 1.62x | 0.90x | 1.99x | 1.79x | 8/10 |
| corpus_materialize | 0.98x | 1.36x | 1.24x | 1.02x | 1.65x | 1.11x | 9/10 |
| corpus_uci_sorted | 1.03x | 2.34x | 3.16x | 2.32x | 3.13x | 1.57x | 10/10 |
| replay_trusted_native | 0.96x | 0.88x | 1.53x | 1.58x | 2.27x | 2.57x | 9/10 |
| replay_checked_native | 0.89x | 1.84x | 0.93x | 7.59x | 7.43x | 8.62x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
