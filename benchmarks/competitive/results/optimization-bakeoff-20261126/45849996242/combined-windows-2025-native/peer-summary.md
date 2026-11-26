| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.84x | 1.78x | 1.72x | 2.74x | 2.16x | 30/30 |
| materialize | 0.99x | 2.01x | 1.36x | 1.14x | 2.33x | 1.51x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.90x | 1.61x | 1.42x | 2.45x | 2.21x | 30/30 |
| perft_full_d3 | 1.09x | 1.66x | 1.68x | 1.41x | 2.28x | 2.01x | 30/30 |
| fen_parse | 0.92x | 1.35x | 1.43x | 1.38x | 1.90x | 1.45x | 30/30 |
| fen_write_native_ep | 0.99x | 3.99x | 7.59x | 1.70x | 31.51x | 2.08x | 30/30 |
| snapshot | 0.98x | 1.81x | 1.26x | 4.18x | 1.26x | 1.12x | 30/30 |
| san_parse_all | 0.95x | 1.81x | N/A | 4.42x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.25x | N/A | 3.32x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.91x | 1.28x | 7.94x | 1.50x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 87.16x | 132.71x | 196.68x | 151.35x | 5/5 |
| chess960_materialize | 0.97x | 1.62x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.93x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.50x | 1.78x | 1.30x | 2.38x | 2.13x | 9/10 |
| corpus_materialize | 1.00x | 1.90x | 1.39x | 1.19x | 2.82x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.90x | 2.43x | 1.99x | 2.31x | 1.50x | 10/10 |
| replay_trusted_native | 1.11x | 0.97x | 1.73x | 2.10x | 3.40x | 4.14x | 9/10 |
| replay_checked_native | 1.14x | 2.31x | 1.02x | 5.73x | 6.74x | 7.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
