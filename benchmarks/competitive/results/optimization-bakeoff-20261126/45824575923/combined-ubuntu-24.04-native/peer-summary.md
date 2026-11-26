| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 4.27x | 1.84x | 1.55x | 2.46x | 1.88x | 30/30 |
| materialize | 0.96x | 2.09x | 1.62x | 1.68x | 2.53x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.16x | 3.77x | 1.54x | 1.17x | 2.33x | 1.85x | 29/30 |
| perft_full_d3 | 0.99x | 1.50x | 1.64x | 1.59x | 2.16x | 1.79x | 30/30 |
| fen_parse | 1.08x | 1.36x | 1.34x | 1.06x | 1.47x | 1.28x | 30/30 |
| fen_write_native_ep | 0.97x | 4.70x | 9.91x | 2.03x | 27.93x | 2.56x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 0.89x | 1.73x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.96x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.07x | 1.30x | 3.91x | 1.39x | 1.59x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 78.62x | 137.93x | 170.76x | 156.79x | 4/5 |
| chess960_materialize | 1.06x | 1.73x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.50x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 40.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.87x | 17.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.27x | 5.42x | 2.02x | 1.11x | 2.47x | 2.22x | 9/10 |
| corpus_materialize | 1.05x | 1.42x | 1.31x | 1.07x | 1.72x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.26x | 3.00x | 2.17x | 2.94x | 1.53x | 10/10 |
| replay_trusted_native | 1.00x | 0.90x | 1.60x | 1.56x | 2.21x | 2.59x | 9/10 |
| replay_checked_native | 1.02x | 1.96x | 1.08x | 8.27x | 8.04x | 9.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
