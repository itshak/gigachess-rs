| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.35x | 1.56x | 1.51x | 2.37x | 1.90x | 30/30 |
| materialize | 1.01x | 1.96x | 1.30x | 1.09x | 2.20x | 1.39x | 29/30 |
| perft_bulk_d4 | 0.99x | 3.73x | 1.53x | 1.34x | 2.32x | 2.09x | 30/30 |
| perft_full_d3 | 0.96x | 1.25x | 1.29x | 1.03x | 1.78x | 1.59x | 28/30 |
| fen_parse | 0.99x | 1.43x | 1.60x | 1.51x | 2.38x | 1.56x | 30/30 |
| fen_write_native_ep | 1.01x | 3.99x | 7.86x | 1.60x | 31.58x | 2.04x | 30/30 |
| snapshot | 0.82x | 1.63x | 1.19x | 2.84x | 1.22x | 1.11x | 30/30 |
| san_parse_all | 1.00x | 1.89x | N/A | 4.56x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.70x | N/A | 3.45x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.55x | 1.06x | 6.28x | 1.19x | 1.39x | 28/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.08x | 81.79x | 134.40x | 202.07x | 145.16x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.22x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.57x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 13.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.41x | 1.81x | 1.31x | 2.39x | 2.16x | 10/10 |
| corpus_materialize | 1.02x | 1.78x | 1.37x | 1.16x | 2.79x | 1.47x | 10/10 |
| corpus_uci_sorted | 1.03x | 1.88x | 2.39x | 1.96x | 2.30x | 1.53x | 10/10 |
| replay_trusted_native | 0.97x | 0.77x | 1.30x | 1.64x | 2.65x | 3.51x | 9/10 |
| replay_checked_native | 1.07x | 2.11x | 0.92x | 5.24x | 6.67x | 7.09x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
