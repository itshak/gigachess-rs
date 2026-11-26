| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.75x | 1.71x | 1.64x | 2.57x | 1.98x | 30/30 |
| materialize | 1.04x | 2.15x | 1.75x | 1.17x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.32x | 1.67x | 1.36x | 2.63x | 2.30x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.21x | 0.94x | 1.74x | 1.46x | 25/30 |
| fen_parse | 0.94x | 1.30x | 1.43x | 1.09x | 1.90x | 1.35x | 29/30 |
| fen_write_native_ep | 1.00x | 4.01x | 10.03x | 1.92x | 28.46x | 2.13x | 30/30 |
| snapshot | 0.53x | 1.00x | 0.66x | 2.00x | 0.66x | 0.66x | 9/30 |
| san_parse_all | 1.00x | 2.27x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.26x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.91x | 1.43x | 4.57x | 1.74x | 2.04x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 103.67x | 137.54x | 217.43x | 162.95x | 5/5 |
| chess960_materialize | 0.99x | 1.89x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.08x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.58x | 0.58x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 19.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.41x | 1.76x | 1.30x | 2.34x | 2.03x | 9/10 |
| corpus_materialize | 1.01x | 1.91x | 1.78x | 1.27x | 2.88x | 1.60x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.32x | 3.16x | 2.31x | 2.93x | 1.65x | 10/10 |
| replay_trusted_native | 0.97x | 1.11x | 1.96x | 2.17x | 3.74x | 4.56x | 10/10 |
| replay_checked_native | 0.99x | 2.62x | 1.02x | 5.74x | 6.47x | 7.73x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
