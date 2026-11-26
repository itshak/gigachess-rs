| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.37x | 1.68x | 1.79x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.93x | 1.53x | 1.33x | 2.15x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.59x | 1.42x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.60x | 1.34x | 1.97x | 1.85x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.29x | 1.73x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.47x | 9.31x | 1.58x | 29.93x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.12x | 1.01x | 1.49x | 1.00x | 0.93x | 20/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 12.40x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.03x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.48x | 130.75x | 168.94x | 138.42x | 5/5 |
| chess960_materialize | 0.99x | 1.70x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.98x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.44x | 1.72x | 1.36x | 1.99x | 2.19x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.54x | 1.23x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.11x | 2.93x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.63x | 1.66x | 2.02x | 2.25x | 2.30x | 8/10 |
| replay_checked_native | 1.07x | 2.33x | 1.14x | 7.62x | 6.83x | 8.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
