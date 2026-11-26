| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.71x | 1.45x | 2.22x | 1.75x | 30/30 |
| materialize | 1.04x | 2.09x | 1.66x | 1.64x | 2.57x | 1.79x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.59x | 1.21x | 2.38x | 1.92x | 30/30 |
| perft_full_d3 | 1.10x | 1.47x | 1.73x | 1.65x | 2.12x | 1.77x | 30/30 |
| fen_parse | 1.05x | 1.29x | 1.32x | 1.02x | 1.43x | 1.21x | 28/30 |
| fen_write_native_ep | 1.00x | 5.06x | 10.33x | 1.92x | 29.22x | 2.69x | 30/30 |
| snapshot | 1.10x | 1.41x | 1.15x | 3.22x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 0.94x | 1.81x | N/A | 4.04x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.93x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.30x | 1.73x | 1.09x | 3.17x | 1.12x | 1.34x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.21x | 81.15x | 138.18x | 174.29x | 156.52x | 5/5 |
| chess960_materialize | 0.99x | 1.65x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.28x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.17x | 22.38x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.45x | 2.44x | 1.34x | 2.93x | 2.65x | 10/10 |
| corpus_materialize | 1.03x | 1.46x | 1.35x | 1.11x | 1.76x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.25x | 2.92x | 2.22x | 2.88x | 1.54x | 10/10 |
| replay_trusted_native | 0.99x | 0.71x | 1.31x | 1.25x | 1.79x | 2.08x | 8/10 |
| replay_checked_native | 1.02x | 2.08x | 1.09x | 8.65x | 8.29x | 10.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
