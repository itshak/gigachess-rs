| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.11x | 1.76x | 1.53x | 2.41x | 1.83x | 30/30 |
| materialize | 0.97x | 2.03x | 1.46x | 1.62x | 2.44x | 1.64x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.07x | 1.68x | 1.27x | 2.24x | 1.97x | 30/30 |
| perft_full_d3 | 0.96x | 1.64x | 1.86x | 1.67x | 2.38x | 1.94x | 30/30 |
| fen_parse | 0.87x | 1.24x | 1.35x | 1.33x | 1.71x | 1.40x | 30/30 |
| fen_write_native_ep | 0.98x | 4.26x | 7.81x | 1.70x | 30.56x | 2.17x | 30/30 |
| snapshot | 1.62x | 1.75x | 0.99x | 3.74x | 1.02x | 0.99x | 20/30 |
| san_parse_all | 1.00x | 1.76x | N/A | 5.31x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.77x | N/A | 4.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.61x | 1.10x | 8.21x | 1.13x | 1.28x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.06x | N/A | N/A | 4/6 |
| startpos | 1.00x | 0.99x | 68.77x | 140.17x | 179.26x | 146.97x | 4/5 |
| chess960_materialize | 1.00x | 1.52x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.32x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 31.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 24.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.39x | 2.61x | 1.33x | 2.86x | 2.79x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.25x | 1.15x | 1.80x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.84x | 2.19x | 1.87x | 2.23x | 1.55x | 10/10 |
| replay_trusted_native | 0.98x | 0.67x | 1.23x | 1.36x | 1.76x | 2.02x | 8/10 |
| replay_checked_native | 0.96x | 2.04x | 1.06x | 8.44x | 7.74x | 9.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
