| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.28x | 1.76x | 1.80x | 2.89x | 2.10x | 30/30 |
| materialize | 1.01x | 2.02x | 1.66x | 1.29x | 2.34x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.30x | 1.63x | 1.50x | 2.64x | 1.99x | 30/30 |
| perft_full_d3 | 1.21x | 1.63x | 1.81x | 1.61x | 2.37x | 2.19x | 30/30 |
| fen_parse | 1.00x | 1.45x | 1.57x | 1.24x | 2.27x | 1.36x | 30/30 |
| fen_write_native_ep | 0.96x | 4.35x | 9.74x | 1.71x | 27.94x | 2.13x | 30/30 |
| snapshot | 0.78x | 1.54x | 0.97x | 2.55x | 1.18x | 0.98x | 18/30 |
| san_parse_all | 1.00x | 2.17x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.64x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.32x | 1.32x | 5.03x | 1.47x | 1.61x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.84x | 143.53x | 227.66x | 150.40x | 5/5 |
| chess960_materialize | 0.98x | 1.61x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.91x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 29.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 21.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.21x | 1.96x | 1.57x | 2.99x | 2.57x | 10/10 |
| corpus_materialize | 0.99x | 1.61x | 1.54x | 1.18x | 2.12x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.51x | 3.25x | 2.44x | 3.17x | 1.81x | 10/10 |
| replay_trusted_native | 0.96x | 0.87x | 1.55x | 1.88x | 2.52x | 2.73x | 9/10 |
| replay_checked_native | 0.96x | 2.60x | 1.08x | 8.30x | 8.24x | 9.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
