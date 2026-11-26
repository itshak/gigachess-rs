| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 0.98x | 1.46x | 1.58x | 1.31x | 1.93x | 1.83x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.25x | 1.73x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.37x | 1.57x | 29.79x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.11x | 1.05x | 1.47x | 1.05x | 0.95x | 24/30 |
| san_parse_all | 0.97x | 1.81x | N/A | 4.17x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.57x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.02x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.54x | 127.46x | 168.15x | 137.98x | 5/5 |
| chess960_materialize | 0.99x | 1.70x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.92x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.36x | 1.67x | 1.34x | 1.95x | 2.15x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.52x | 1.22x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.11x | 2.92x | 2.08x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 0.93x | 0.60x | 1.54x | 1.88x | 2.08x | 2.13x | 8/10 |
| replay_checked_native | 0.98x | 2.27x | 1.10x | 7.43x | 6.66x | 8.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
