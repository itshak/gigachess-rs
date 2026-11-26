| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.28x | 1.79x | 1.56x | 2.40x | 1.83x | 30/30 |
| materialize | 1.05x | 2.21x | 1.62x | 1.72x | 2.65x | 1.89x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.70x | 1.47x | 1.15x | 2.28x | 1.79x | 29/30 |
| perft_full_d3 | 0.91x | 1.23x | 1.42x | 1.37x | 1.85x | 1.46x | 30/30 |
| fen_parse | 1.00x | 1.47x | 1.57x | 1.18x | 1.46x | 1.13x | 30/30 |
| fen_write_native_ep | 0.89x | 4.46x | 9.22x | 1.69x | 25.42x | 2.22x | 30/30 |
| snapshot | 0.75x | 1.37x | 0.87x | 3.24x | 0.87x | 1.00x | 15/30 |
| san_parse_all | 1.01x | 1.88x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.10x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.13x | 1.37x | 3.88x | 1.37x | 1.63x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.73x | 0.91x | 71.98x | 126.02x | 155.24x | 127.46x | 4/5 |
| chess960_materialize | 0.89x | 1.64x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.43x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.95x | 22.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.25x | 1.95x | 1.09x | 2.39x | 2.20x | 9/10 |
| corpus_materialize | 1.01x | 1.42x | 1.29x | 1.07x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.35x | 2.99x | 2.25x | 2.94x | 1.57x | 10/10 |
| replay_trusted_native | 0.94x | 0.83x | 1.53x | 1.52x | 2.09x | 2.40x | 9/10 |
| replay_checked_native | 0.88x | 1.93x | 1.04x | 8.04x | 7.79x | 9.15x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
