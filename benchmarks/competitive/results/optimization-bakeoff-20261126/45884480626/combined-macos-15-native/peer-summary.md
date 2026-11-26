| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 5.20x | 1.77x | 1.91x | 2.41x | 2.10x | 30/30 |
| materialize | 0.93x | 2.00x | 1.56x | 1.12x | 2.12x | 1.39x | 26/30 |
| perft_bulk_d4 | 0.84x | 4.09x | 1.60x | 1.21x | 2.05x | 1.86x | 29/30 |
| perft_full_d3 | 1.06x | 1.88x | 2.00x | 1.43x | 2.23x | 2.24x | 30/30 |
| fen_parse | 1.07x | 1.32x | 1.45x | 1.40x | 1.90x | 1.79x | 30/30 |
| fen_write_native_ep | 0.97x | 4.02x | 8.36x | 1.72x | 28.00x | 2.55x | 30/30 |
| snapshot | 0.93x | 1.65x | 1.19x | 2.44x | 1.19x | 0.89x | 24/30 |
| san_parse_all | 0.96x | 1.77x | N/A | 3.83x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 11.44x | N/A | 2.63x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.18x | 1.36x | 6.74x | 1.21x | 1.38x | 27/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.86x | 1.26x | 83.08x | 152.41x | 180.20x | 176.82x | 5/5 |
| chess960_materialize | 0.93x | 1.64x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 3.16x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.19x | 38.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.17x | 1.04x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.13x | 22.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.17x | 2.24x | 1.19x | 2.59x | 2.43x | 10/10 |
| corpus_materialize | 1.10x | 1.47x | 1.35x | 1.25x | 1.97x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.28x | 1.97x | 2.81x | 2.24x | 2.68x | 1.87x | 10/10 |
| replay_trusted_native | 0.99x | 0.57x | 1.44x | 1.41x | 1.52x | 1.60x | 8/10 |
| replay_checked_native | 1.34x | 2.63x | 1.34x | 9.59x | 8.37x | 10.79x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
