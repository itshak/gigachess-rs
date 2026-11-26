| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.36x | 1.53x | 1.53x | 2.16x | 1.86x | 30/30 |
| materialize | 1.02x | 1.89x | 1.29x | 1.18x | 2.30x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.57x | 1.40x | 1.22x | 2.02x | 1.80x | 30/30 |
| perft_full_d3 | 0.97x | 1.35x | 1.43x | 1.26x | 1.88x | 1.78x | 30/30 |
| fen_parse | 1.19x | 1.35x | 1.57x | 1.36x | 2.37x | 1.61x | 30/30 |
| fen_write_native_ep | 0.99x | 4.08x | 7.98x | 1.61x | 32.81x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.03x | 4.19x | 1.15x | 1.01x | 23/30 |
| san_parse_all | 1.02x | 2.08x | N/A | 5.66x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.95x | N/A | 3.88x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.26x | 1.36x | 9.85x | 1.45x | 1.64x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 81.41x | 151.71x | 240.27x | 179.56x | 5/5 |
| chess960_materialize | 1.00x | 1.55x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.73x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 22.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.92x | 1.60x | 1.22x | 2.17x | 2.14x | 10/10 |
| corpus_materialize | 1.03x | 1.53x | 1.20x | 1.11x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.76x | 2.18x | 1.83x | 2.16x | 1.42x | 10/10 |
| replay_trusted_native | 1.00x | 0.84x | 1.58x | 2.06x | 2.59x | 3.00x | 9/10 |
| replay_checked_native | 1.07x | 2.43x | 1.08x | 8.27x | 8.11x | 10.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
