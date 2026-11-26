| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.78x | 1.55x | 2.67x | 1.81x | 30/30 |
| materialize | 1.07x | 2.21x | 1.64x | 1.74x | 2.68x | 1.93x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.02x | 1.60x | 1.26x | 2.65x | 1.96x | 30/30 |
| perft_full_d3 | 1.28x | 1.50x | 1.66x | 1.66x | 2.16x | 1.77x | 30/30 |
| fen_parse | 0.97x | 1.31x | 1.30x | 1.07x | 1.50x | 1.18x | 30/30 |
| fen_write_native_ep | 1.01x | 4.60x | 10.33x | 1.93x | 30.71x | 2.57x | 30/30 |
| snapshot | 1.29x | 1.82x | 1.16x | 4.33x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.10x | 2.02x | N/A | 4.17x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.14x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.76x | 1.11x | 3.35x | 1.11x | 1.40x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.12x | 88.56x | 162.41x | 205.51x | 172.75x | 5/5 |
| chess960_materialize | 1.07x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.37x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 16.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.80x | 2.55x | 1.41x | 3.09x | 2.78x | 10/10 |
| corpus_materialize | 1.01x | 1.47x | 1.37x | 1.13x | 1.78x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 3.05x | 2.31x | 2.99x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.73x | 1.26x | 1.29x | 1.81x | 2.07x | 8/10 |
| replay_checked_native | 1.02x | 2.01x | 1.04x | 8.59x | 8.31x | 9.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
