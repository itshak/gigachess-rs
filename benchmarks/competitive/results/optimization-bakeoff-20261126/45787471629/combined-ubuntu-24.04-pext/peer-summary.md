| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.69x | 1.61x | 1.63x | 2.58x | 1.87x | 30/30 |
| materialize | 0.96x | 2.05x | 1.61x | 1.19x | 2.13x | 1.43x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.15x | 1.60x | 1.34x | 2.57x | 2.10x | 30/30 |
| perft_full_d3 | 0.74x | 0.92x | 0.96x | 0.74x | 1.36x | 1.10x | 15/30 |
| fen_parse | 0.96x | 1.30x | 1.42x | 1.10x | 2.05x | 1.24x | 30/30 |
| fen_write_native_ep | 0.99x | 3.96x | 9.46x | 1.81x | 27.16x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.20x | 2.77x | 2.78x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 2.26x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.17x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.55x | 1.05x | 3.74x | 1.16x | 1.35x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.18x | 91.84x | 133.21x | 219.94x | 145.88x | 5/5 |
| chess960_materialize | 0.93x | 1.68x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.93x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.03x | 1.91x | 1.46x | 2.68x | 2.18x | 10/10 |
| corpus_materialize | 1.00x | 2.00x | 1.77x | 1.27x | 2.93x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.35x | 3.18x | 2.36x | 3.04x | 1.67x | 10/10 |
| replay_trusted_native | 1.00x | 0.75x | 1.27x | 1.45x | 2.62x | 3.03x | 8/10 |
| replay_checked_native | 0.97x | 2.36x | 0.91x | 5.29x | 6.01x | 6.95x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
