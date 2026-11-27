| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.88x | 1.65x | 1.43x | 1.90x | 1.36x | 30/30 |
| materialize | 1.20x | 2.31x | 1.88x | 1.93x | 2.86x | 2.03x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.00x | 1.60x | 1.20x | 1.99x | 1.78x | 29/30 |
| perft_full_d3 | 1.01x | 1.37x | 1.61x | 1.41x | 1.92x | 1.60x | 30/30 |
| fen_parse | 1.00x | 1.71x | 1.45x | 1.16x | 1.60x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 3.00x | 9.57x | 1.78x | 27.21x | 2.28x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.24x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.91x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.31x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.20x | 1.43x | 4.10x | 1.36x | 1.55x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.29x | 58.73x | 107.54x | 127.85x | 102.06x | 5/5 |
| chess960_materialize | 1.01x | 1.82x | 1.76x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.11x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 40.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.07x | 20.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.27x | 2.14x | 1.17x | 2.20x | 2.94x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.41x | 1.14x | 1.83x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 2.96x | 2.24x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 0.92x | 0.82x | 1.52x | 1.53x | 2.08x | 1.89x | 9/10 |
| replay_checked_native | 0.99x | 2.20x | 1.15x | 8.77x | 8.68x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
