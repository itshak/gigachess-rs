| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.16x | 5.35x | 1.78x | 1.81x | 2.40x | 2.18x | 30/30 |
| materialize | 1.11x | 2.18x | 1.81x | 1.20x | 2.49x | 1.45x | 29/30 |
| perft_bulk_d4 | 1.16x | 4.27x | 1.58x | 1.34x | 2.17x | 2.08x | 30/30 |
| perft_full_d3 | 1.05x | 1.63x | 1.68x | 1.25x | 1.88x | 1.97x | 30/30 |
| fen_parse | 1.04x | 1.18x | 1.36x | 1.21x | 1.50x | 1.34x | 30/30 |
| fen_write_native_ep | 1.15x | 4.52x | 9.38x | 1.81x | 28.25x | 2.38x | 30/30 |
| snapshot | 1.08x | 1.73x | 1.28x | 2.61x | 1.14x | 1.17x | 27/30 |
| san_parse_all | 1.05x | 1.70x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 12.02x | N/A | 2.84x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.13x | 1.19x | 5.49x | 1.18x | 1.29x | 27/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.82x | 1.38x | 60.98x | 122.66x | 157.05x | 111.29x | 5/5 |
| chess960_materialize | 0.77x | 1.79x | 1.68x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.76x | 3.26x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 19.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.55x | 2.23x | 1.43x | 2.64x | 2.96x | 10/10 |
| corpus_materialize | 1.05x | 1.53x | 1.42x | 1.19x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.83x | 2.58x | 1.87x | 2.56x | 1.42x | 10/10 |
| replay_trusted_native | 1.03x | 0.63x | 1.54x | 1.73x | 1.74x | 1.78x | 8/10 |
| replay_checked_native | 0.98x | 2.66x | 1.20x | 9.13x | 8.31x | 10.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
