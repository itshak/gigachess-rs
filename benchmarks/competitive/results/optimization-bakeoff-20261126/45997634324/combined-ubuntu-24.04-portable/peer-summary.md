| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 3.25x | 1.65x | 1.50x | 2.12x | 1.48x | 30/30 |
| materialize | 1.00x | 2.21x | 1.89x | 1.36x | 2.53x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.32x | 1.80x | 1.40x | 2.25x | 1.95x | 30/30 |
| perft_full_d3 | 1.18x | 1.62x | 1.85x | 1.49x | 2.23x | 2.04x | 30/30 |
| fen_parse | 0.95x | 1.49x | 1.55x | 1.34x | 2.20x | 1.29x | 30/30 |
| fen_write_native_ep | 1.04x | 3.33x | 9.77x | 1.66x | 27.07x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.71x | 1.23x | 3.06x | 1.05x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 2.02x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.30x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.23x | 1.57x | 4.55x | 1.44x | 1.48x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 64.43x | 122.89x | 175.28x | 109.89x | 5/5 |
| chess960_materialize | 1.00x | 1.64x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.39x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 23.55x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 18.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.21x | 2.12x | 1.41x | 2.33x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.62x | 1.17x | 2.07x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.27x | 3.25x | 2.50x | 3.14x | 1.83x | 10/10 |
| replay_trusted_native | 1.00x | 0.89x | 1.71x | 2.02x | 2.68x | 2.49x | 9/10 |
| replay_checked_native | 0.97x | 2.26x | 1.10x | 7.66x | 7.95x | 8.81x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
