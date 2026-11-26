| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.99x | 1.68x | 1.66x | 2.44x | 2.01x | 30/30 |
| materialize | 1.00x | 2.19x | 1.64x | 1.22x | 2.45x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.24x | 1.59x | 1.29x | 2.47x | 2.23x | 30/30 |
| perft_full_d3 | 1.01x | 1.21x | 1.42x | 1.09x | 1.84x | 1.74x | 29/30 |
| fen_parse | 1.11x | 1.43x | 1.56x | 1.30x | 2.19x | 1.28x | 30/30 |
| fen_write_native_ep | 1.00x | 3.89x | 8.55x | 1.52x | 25.36x | 1.99x | 30/30 |
| snapshot | 1.00x | 2.50x | 1.50x | 4.50x | 1.50x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 1.78x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.70x | N/A | 2.24x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.95x | 1.45x | 4.25x | 1.39x | 1.95x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.20x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 84.72x | 145.18x | 228.32x | 141.96x | 5/5 |
| chess960_materialize | 1.04x | 1.77x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.94x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 35.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 27.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.01x | 1.73x | 1.17x | 2.27x | 2.12x | 9/10 |
| corpus_materialize | 0.97x | 1.68x | 1.55x | 1.14x | 2.22x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.36x | 2.95x | 2.38x | 2.99x | 1.69x | 10/10 |
| replay_trusted_native | 1.03x | 0.73x | 1.40x | 1.57x | 2.51x | 2.83x | 9/10 |
| replay_checked_native | 1.04x | 2.66x | 1.21x | 9.10x | 9.26x | 11.22x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
