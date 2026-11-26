| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.88x | 4.17x | 1.48x | 1.50x | 2.00x | 1.78x | 30/30 |
| materialize | 0.95x | 1.77x | 1.39x | 0.94x | 2.27x | 1.34x | 26/30 |
| perft_bulk_d4 | 1.03x | 3.72x | 1.37x | 1.10x | 1.70x | 1.65x | 28/30 |
| perft_full_d3 | 1.16x | 1.65x | 1.84x | 1.41x | 2.02x | 2.00x | 29/30 |
| fen_parse | 1.06x | 1.32x | 1.33x | 1.38x | 1.64x | 1.43x | 30/30 |
| fen_write_native_ep | 1.08x | 4.46x | 9.58x | 1.82x | 27.30x | 2.31x | 30/30 |
| snapshot | 1.02x | 1.66x | 1.13x | 2.35x | 1.14x | 0.97x | 24/30 |
| san_parse_all | 1.03x | 1.81x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 12.25x | N/A | 3.00x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.11x | 1.17x | 5.53x | 1.11x | 1.20x | 26/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.79x | 1.11x | 74.74x | 145.83x | 170.29x | 183.34x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.29x | 2.57x | 1.19x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.88x | 39.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.88x | 4.46x | 1.60x | 0.93x | 1.78x | 2.04x | 9/10 |
| corpus_materialize | 0.99x | 1.45x | 1.39x | 1.12x | 1.78x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.79x | 1.77x | 2.56x | 1.64x | 2.45x | 1.35x | 10/10 |
| replay_trusted_native | 0.87x | 0.66x | 1.81x | 1.65x | 1.93x | 1.64x | 8/10 |
| replay_checked_native | 0.87x | 2.51x | 1.27x | 8.98x | 8.57x | 9.71x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
