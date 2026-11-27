| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.80x | 1.78x | 1.71x | 2.70x | 2.18x | 30/30 |
| materialize | 1.03x | 2.09x | 1.36x | 1.14x | 2.34x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.00x | 1.63x | 1.47x | 2.47x | 2.28x | 30/30 |
| perft_full_d3 | 1.01x | 1.50x | 1.57x | 1.30x | 2.09x | 1.85x | 30/30 |
| fen_parse | 1.11x | 1.55x | 1.64x | 1.59x | 2.39x | 1.63x | 30/30 |
| fen_write_native_ep | 1.01x | 4.01x | 7.77x | 1.60x | 31.38x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.25x | 4.10x | 1.25x | 1.17x | 30/30 |
| san_parse_all | 1.03x | 1.99x | N/A | 4.75x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.11x | N/A | 3.25x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.96x | 1.32x | 8.27x | 1.54x | 1.74x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 84.72x | 137.99x | 196.19x | 144.11x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.89x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.67x | 1.79x | 1.28x | 2.38x | 2.15x | 9/10 |
| corpus_materialize | 1.02x | 1.82x | 1.39x | 1.19x | 2.82x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.89x | 2.41x | 1.99x | 2.29x | 1.50x | 10/10 |
| replay_trusted_native | 1.04x | 1.12x | 1.93x | 2.42x | 3.84x | 4.76x | 10/10 |
| replay_checked_native | 0.99x | 2.47x | 1.09x | 6.51x | 7.48x | 8.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
