| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 4.47x | 1.88x | 1.62x | 2.27x | 2.03x | 30/30 |
| materialize | 1.00x | 2.00x | 1.64x | 1.63x | 2.32x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.99x | 1.63x | 1.24x | 2.26x | 1.97x | 29/30 |
| perft_full_d3 | 1.26x | 1.71x | 1.99x | 1.88x | 2.38x | 2.12x | 30/30 |
| fen_parse | 0.92x | 1.24x | 1.45x | 1.07x | 1.39x | 1.14x | 30/30 |
| fen_write_native_ep | 1.01x | 4.97x | 10.26x | 2.18x | 29.04x | 2.67x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.05x | 1.84x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.44x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.34x | 4.11x | 1.25x | 1.74x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.25x | 101.93x | 173.84x | 221.51x | 179.90x | 5/5 |
| chess960_materialize | 1.05x | 1.63x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.56x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.60x | 1.69x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 25.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.40x | 6.47x | 2.55x | 1.30x | 2.90x | 2.84x | 10/10 |
| corpus_materialize | 0.99x | 1.36x | 1.28x | 1.04x | 1.63x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.25x | 2.89x | 2.18x | 2.79x | 1.57x | 10/10 |
| replay_trusted_native | 1.04x | 0.89x | 1.67x | 1.56x | 2.16x | 2.53x | 9/10 |
| replay_checked_native | 1.00x | 2.29x | 1.23x | 9.35x | 8.96x | 10.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
