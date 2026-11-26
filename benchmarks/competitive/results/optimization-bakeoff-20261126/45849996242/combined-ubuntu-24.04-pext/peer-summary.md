| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.49x | 1.88x | 1.64x | 2.59x | 1.91x | 30/30 |
| materialize | 1.00x | 2.34x | 1.68x | 1.76x | 2.82x | 2.04x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.82x | 1.52x | 1.19x | 2.38x | 1.86x | 29/30 |
| perft_full_d3 | 1.14x | 1.54x | 1.78x | 1.71x | 2.29x | 1.87x | 30/30 |
| fen_parse | 0.94x | 1.30x | 1.28x | 1.06x | 1.37x | 1.07x | 29/30 |
| fen_write_native_ep | 0.99x | 4.55x | 10.15x | 2.10x | 28.10x | 2.50x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.10x | 1.93x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 8.89x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.31x | 3.95x | 1.41x | 1.61x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.90x | 0.99x | 79.61x | 147.81x | 171.12x | 141.21x | 4/5 |
| chess960_materialize | 1.01x | 1.85x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.49x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 38.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.82x | 19.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.46x | 2.03x | 1.13x | 2.53x | 2.25x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.30x | 1.09x | 1.76x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.93x | 2.20x | 2.88x | 1.55x | 10/10 |
| replay_trusted_native | 0.97x | 0.88x | 1.62x | 1.56x | 2.23x | 2.48x | 9/10 |
| replay_checked_native | 1.01x | 2.01x | 1.12x | 8.55x | 8.36x | 9.64x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
