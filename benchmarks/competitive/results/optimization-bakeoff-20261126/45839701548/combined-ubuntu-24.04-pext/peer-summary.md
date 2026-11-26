| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.47x | 1.87x | 1.63x | 2.49x | 1.91x | 30/30 |
| materialize | 0.95x | 2.15x | 1.59x | 1.72x | 2.63x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.79x | 1.50x | 1.18x | 2.33x | 1.95x | 29/30 |
| perft_full_d3 | 1.00x | 1.29x | 1.52x | 1.42x | 1.88x | 1.56x | 30/30 |
| fen_parse | 1.00x | 1.39x | 1.28x | 1.12x | 1.38x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 4.86x | 10.55x | 2.10x | 27.90x | 2.53x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.16x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.96x | 1.85x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.39x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.32x | 3.90x | 1.29x | 1.62x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.43x | 158.13x | 192.79x | 174.92x | 5/5 |
| chess960_materialize | 1.00x | 1.74x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 37.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.63x | 0.61x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.97x | 18.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.11x | 1.89x | 1.05x | 2.32x | 2.14x | 9/10 |
| corpus_materialize | 1.00x | 1.42x | 1.29x | 1.09x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.26x | 2.93x | 2.18x | 2.90x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.87x | 1.60x | 1.57x | 2.20x | 2.49x | 9/10 |
| replay_checked_native | 0.97x | 2.02x | 1.07x | 8.32x | 8.05x | 9.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
