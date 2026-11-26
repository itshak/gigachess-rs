| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.78x | 1.55x | 2.45x | 1.83x | 30/30 |
| materialize | 0.99x | 2.09x | 1.60x | 1.71x | 2.52x | 1.83x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.05x | 1.60x | 1.26x | 2.50x | 1.98x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.80x | 1.70x | 2.26x | 2.43x | 30/30 |
| fen_parse | 1.01x | 1.29x | 1.28x | 1.00x | 1.39x | 1.17x | 28/30 |
| fen_write_native_ep | 1.00x | 3.10x | 6.65x | 1.21x | 18.39x | 1.70x | 30/30 |
| snapshot | 2.00x | 1.50x | 1.17x | 2.00x | 1.16x | 1.00x | 26/30 |
| san_parse_all | 0.99x | 1.92x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.32x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.79x | 1.18x | 3.20x | 1.15x | 1.38x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 67.83x | 114.81x | 150.89x | 132.77x | 5/5 |
| chess960_materialize | 0.99x | 1.74x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.38x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 19.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.85x | 2.55x | 1.40x | 3.13x | 2.84x | 10/10 |
| corpus_materialize | 1.00x | 1.47x | 1.35x | 1.12x | 1.80x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 2.96x | 2.23x | 2.90x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.31x | 1.26x | 1.79x | 2.05x | 8/10 |
| replay_checked_native | 1.00x | 1.99x | 1.11x | 8.42x | 8.17x | 9.57x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
