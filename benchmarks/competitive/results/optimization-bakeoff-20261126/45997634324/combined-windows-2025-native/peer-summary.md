| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.13x | 1.81x | 1.77x | 2.55x | 2.25x | 30/30 |
| materialize | 1.00x | 2.12x | 1.39x | 1.28x | 2.39x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.18x | 1.63x | 1.45x | 2.40x | 2.21x | 30/30 |
| perft_full_d3 | 1.19x | 1.74x | 1.83x | 1.62x | 2.34x | 2.33x | 30/30 |
| fen_parse | 1.08x | 1.46x | 1.64x | 1.41x | 2.41x | 1.50x | 30/30 |
| fen_write_native_ep | 0.94x | 3.87x | 7.58x | 1.49x | 29.84x | 1.91x | 30/30 |
| snapshot | 0.96x | 2.00x | 1.02x | 3.96x | 1.07x | 0.99x | 22/30 |
| san_parse_all | 1.05x | 2.10x | N/A | 5.66x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.23x | N/A | 3.87x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.29x | 1.41x | 9.63x | 1.49x | 1.65x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 81.22x | 156.07x | 236.87x | 167.37x | 5/5 |
| chess960_materialize | 0.98x | 1.58x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.82x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 27.20x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.92x | 20.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.66x | 2.00x | 1.45x | 2.63x | 2.46x | 10/10 |
| corpus_materialize | 0.98x | 1.54x | 1.28x | 1.16x | 2.01x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.85x | 2.30x | 1.90x | 2.26x | 1.38x | 10/10 |
| replay_trusted_native | 0.91x | 0.78x | 1.57x | 2.03x | 2.51x | 2.84x | 9/10 |
| replay_checked_native | 0.98x | 2.49x | 1.16x | 9.39x | 8.71x | 10.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
