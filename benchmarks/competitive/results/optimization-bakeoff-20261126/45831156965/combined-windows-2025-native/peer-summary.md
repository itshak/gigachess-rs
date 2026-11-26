| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.74x | 1.58x | 1.36x | 1.84x | 1.63x | 30/30 |
| materialize | 1.00x | 1.94x | 1.34x | 1.46x | 2.30x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.27x | 1.33x | 1.00x | 1.84x | 1.57x | 27/30 |
| perft_full_d3 | 1.00x | 1.60x | 1.78x | 1.62x | 2.27x | 1.87x | 30/30 |
| fen_parse | 1.04x | 1.26x | 1.45x | 1.38x | 1.74x | 1.50x | 30/30 |
| fen_write_native_ep | 0.99x | 3.94x | 7.99x | 1.67x | 30.05x | 2.12x | 30/30 |
| snapshot | 0.99x | 2.07x | 1.18x | 4.87x | 1.16x | 1.13x | 29/30 |
| san_parse_all | 0.99x | 1.75x | N/A | 5.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.56x | N/A | 4.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.35x | 10.29x | 1.31x | 1.63x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.01x | 77.96x | 165.99x | 206.62x | 175.78x | 5/5 |
| chess960_materialize | 0.99x | 1.45x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.32x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 32.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.96x | 1.62x | 0.80x | 1.78x | 1.70x | 8/10 |
| corpus_materialize | 1.01x | 1.33x | 1.15x | 1.02x | 1.65x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.80x | 2.14x | 1.86x | 2.13x | 1.53x | 10/10 |
| replay_trusted_native | 1.06x | 0.85x | 1.60x | 1.71x | 2.21x | 2.60x | 9/10 |
| replay_checked_native | 1.03x | 2.18x | 1.17x | 8.57x | 8.49x | 9.78x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
