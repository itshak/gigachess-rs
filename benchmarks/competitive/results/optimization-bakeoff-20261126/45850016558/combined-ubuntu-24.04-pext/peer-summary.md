| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.49x | 1.87x | 1.63x | 2.57x | 1.90x | 30/30 |
| materialize | 1.00x | 2.31x | 1.76x | 1.75x | 2.79x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.86x | 1.54x | 1.20x | 2.40x | 1.88x | 29/30 |
| perft_full_d3 | 0.97x | 1.31x | 1.54x | 1.49x | 1.92x | 1.57x | 30/30 |
| fen_parse | 0.94x | 1.30x | 1.23x | 1.06x | 1.53x | 1.07x | 30/30 |
| fen_write_native_ep | 0.96x | 4.75x | 9.76x | 2.03x | 28.29x | 2.44x | 30/30 |
| snapshot | 0.71x | 1.58x | 1.25x | 2.83x | 1.25x | 1.15x | 30/30 |
| san_parse_all | 1.12x | 1.95x | N/A | 4.49x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.09x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.15x | 1.39x | 3.99x | 1.39x | 1.70x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.30x | 82.90x | 153.62x | 189.70x | 149.50x | 5/5 |
| chess960_materialize | 0.94x | 1.77x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.75x | 17.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.09x | 1.90x | 1.06x | 2.32x | 2.09x | 10/10 |
| corpus_materialize | 0.99x | 1.42x | 1.28x | 1.08x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.05x | 2.20x | 2.99x | 1.57x | 10/10 |
| replay_trusted_native | 1.02x | 0.92x | 1.70x | 1.66x | 2.33x | 2.62x | 9/10 |
| replay_checked_native | 1.01x | 2.21x | 1.06x | 8.44x | 8.13x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
