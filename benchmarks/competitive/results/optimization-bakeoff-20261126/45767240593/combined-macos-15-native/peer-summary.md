| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.22x | 4.14x | 1.41x | 1.39x | 1.71x | 1.81x | 29/30 |
| materialize | 1.00x | 1.87x | 1.45x | 0.95x | 2.12x | 1.18x | 26/30 |
| perft_bulk_d4 | 1.15x | 3.81x | 1.40x | 1.12x | 1.75x | 1.78x | 29/30 |
| perft_full_d3 | 1.34x | 0.93x | 1.09x | 0.75x | 1.20x | 1.71x | 20/30 |
| fen_parse | 1.10x | 1.27x | 1.36x | 1.38x | 1.68x | 1.52x | 30/30 |
| fen_write_native_ep | 1.01x | 3.16x | 6.71x | 1.35x | 23.41x | 1.75x | 30/30 |
| snapshot | 1.27x | 1.63x | 1.21x | 2.33x | 1.09x | 0.95x | 24/30 |
| san_parse_all | 1.51x | 1.71x | N/A | 3.44x | N/A | N/A | 12/12 |
| san_render_native | 1.81x | 12.08x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.23x | 0.80x | 0.90x | 4.09x | 0.83x | 0.89x | 7/30 |
| make_unmake_full | 1.25x | N/A | N/A | 1.03x | N/A | N/A | 3/6 |
| startpos | 1.45x | 1.19x | 70.22x | 144.92x | 170.82x | 145.41x | 5/5 |
| chess960_materialize | 1.02x | 1.39x | 1.09x | N/A | N/A | N/A | 3/4 |
| chess960_perft_d3 | 1.31x | 2.61x | 1.29x | N/A | N/A | N/A | 3/4 |
| polyglot_key | 1.07x | 39.60x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.19x | 18.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.34x | 6.06x | 2.02x | 1.24x | 2.31x | 2.53x | 10/10 |
| corpus_materialize | 1.12x | 1.53x | 1.41x | 1.11x | 1.89x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.31x | 0.99x | 1.45x | 1.07x | 1.34x | 0.76x | 7/10 |
| replay_trusted_native | 1.27x | 0.44x | 1.11x | 1.31x | 1.21x | 1.29x | 8/10 |
| replay_checked_native | 1.20x | 2.08x | 0.89x | 6.77x | 6.04x | 7.79x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
