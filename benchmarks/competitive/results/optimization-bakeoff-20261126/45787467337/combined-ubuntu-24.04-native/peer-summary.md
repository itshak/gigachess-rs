| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.02x | 1.73x | 1.45x | 2.20x | 1.75x | 30/30 |
| materialize | 1.05x | 2.11x | 1.60x | 1.64x | 2.45x | 1.82x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.93x | 1.59x | 1.21x | 2.37x | 1.90x | 30/30 |
| perft_full_d3 | 0.94x | 1.45x | 1.69x | 1.59x | 2.05x | 1.74x | 30/30 |
| fen_parse | 1.06x | 1.38x | 1.64x | 1.10x | 1.50x | 1.19x | 30/30 |
| fen_write_native_ep | 0.97x | 4.79x | 9.96x | 2.05x | 27.09x | 2.58x | 30/30 |
| snapshot | 0.85x | 1.20x | 1.00x | 2.60x | 1.00x | 1.00x | 18/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.50x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.88x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.63x | 1.04x | 3.14x | 1.02x | 1.28x | 28/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.82x | 1.17x | 66.50x | 115.04x | 146.74x | 121.03x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.25x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 19.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.43x | 2.43x | 1.33x | 2.91x | 2.66x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.33x | 1.08x | 1.74x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.31x | 3.03x | 2.22x | 2.90x | 1.56x | 10/10 |
| replay_trusted_native | 1.02x | 0.72x | 1.34x | 1.28x | 1.82x | 2.14x | 8/10 |
| replay_checked_native | 1.02x | 2.11x | 1.16x | 8.71x | 8.39x | 10.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
