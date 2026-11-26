| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.92x | 1.53x | 1.35x | 1.90x | 1.34x | 30/30 |
| materialize | 1.02x | 1.99x | 1.77x | 1.28x | 2.36x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.79x | 1.49x | 1.18x | 1.90x | 1.63x | 30/30 |
| perft_full_d3 | 1.00x | 1.41x | 1.62x | 1.33x | 1.95x | 1.77x | 30/30 |
| fen_parse | 1.03x | 1.29x | 1.38x | 1.09x | 2.00x | 1.16x | 29/30 |
| fen_write_native_ep | 1.01x | 2.88x | 9.42x | 1.67x | 27.09x | 2.01x | 30/30 |
| snapshot | 1.00x | 2.08x | 1.65x | 3.07x | 1.02x | 1.07x | 30/30 |
| san_parse_all | 1.01x | 2.01x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.36x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.14x | 1.51x | 4.41x | 1.46x | 1.43x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 64.27x | 107.16x | 181.53x | 116.79x | 5/5 |
| chess960_materialize | 1.05x | 1.50x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.15x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 27.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 19.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.59x | 1.77x | 1.20x | 1.96x | 2.38x | 10/10 |
| corpus_materialize | 1.01x | 1.51x | 1.51x | 1.11x | 1.97x | 1.30x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.23x | 3.23x | 2.47x | 3.07x | 1.83x | 10/10 |
| replay_trusted_native | 0.98x | 0.89x | 1.62x | 1.85x | 2.49x | 2.32x | 9/10 |
| replay_checked_native | 0.98x | 2.09x | 1.03x | 7.07x | 7.34x | 8.21x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
