| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.54x | 1.91x | 1.64x | 2.25x | 2.02x | 30/30 |
| materialize | 1.00x | 2.05x | 1.38x | 1.64x | 2.37x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.99x | 1.62x | 1.22x | 2.22x | 1.93x | 29/30 |
| perft_full_d3 | 1.21x | 1.72x | 1.87x | 1.74x | 2.45x | 2.00x | 30/30 |
| fen_parse | 0.88x | 1.07x | 1.36x | 1.10x | 1.65x | 1.33x | 30/30 |
| fen_write_native_ep | 1.00x | 4.05x | 7.82x | 1.71x | 30.22x | 2.16x | 30/30 |
| snapshot | 1.13x | 2.15x | 1.29x | 5.15x | 1.29x | 1.28x | 30/30 |
| san_parse_all | 0.96x | 1.68x | N/A | 5.02x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.82x | N/A | 4.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.36x | 10.61x | 1.27x | 1.68x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 77.88x | 152.06x | 219.91x | 175.05x | 4/5 |
| chess960_materialize | 0.99x | 1.57x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.59x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 41.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 27.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 5.71x | 2.32x | 1.16x | 2.55x | 2.48x | 10/10 |
| corpus_materialize | 1.02x | 1.43x | 1.22x | 1.08x | 1.75x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.88x | 2.24x | 2.06x | 2.19x | 1.63x | 10/10 |
| replay_trusted_native | 0.91x | 0.79x | 1.51x | 1.47x | 2.08x | 2.27x | 8/10 |
| replay_checked_native | 1.00x | 2.30x | 1.17x | 9.15x | 8.95x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
