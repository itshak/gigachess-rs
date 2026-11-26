| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.18x | 1.53x | 1.48x | 2.35x | 1.78x | 30/30 |
| materialize | 1.02x | 2.03x | 1.68x | 1.09x | 2.09x | 1.43x | 28/30 |
| perft_bulk_d4 | 0.97x | 3.75x | 1.48x | 1.21x | 2.34x | 2.00x | 30/30 |
| perft_full_d3 | 1.03x | 1.27x | 1.31x | 0.99x | 1.88x | 1.59x | 26/30 |
| fen_parse | 0.96x | 1.19x | 1.31x | 1.13x | 1.88x | 1.20x | 30/30 |
| fen_write_native_ep | 1.05x | 4.15x | 10.16x | 1.81x | 27.84x | 2.08x | 30/30 |
| snapshot | 1.23x | 1.90x | 1.25x | 3.79x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.03x | 2.26x | N/A | 3.88x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.36x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.75x | 1.86x | 1.23x | 4.41x | 1.41x | 1.58x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.20x | 100.61x | 141.37x | 230.21x | 162.89x | 5/5 |
| chess960_materialize | 1.04x | 1.77x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 3.00x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 15.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 4.04x | 1.60x | 1.18x | 2.18x | 1.88x | 9/10 |
| corpus_materialize | 0.96x | 1.80x | 1.66x | 1.15x | 2.71x | 1.52x | 10/10 |
| corpus_uci_sorted | 0.96x | 2.31x | 3.09x | 2.32x | 2.98x | 1.64x | 10/10 |
| replay_trusted_native | 1.37x | 1.08x | 1.95x | 2.12x | 3.65x | 4.56x | 10/10 |
| replay_checked_native | 1.09x | 2.84x | 1.08x | 6.03x | 6.77x | 8.40x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
