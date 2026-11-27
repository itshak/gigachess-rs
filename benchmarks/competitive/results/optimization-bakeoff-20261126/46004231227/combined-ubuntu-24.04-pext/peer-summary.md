| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.01x | 1.73x | 1.76x | 2.75x | 2.01x | 30/30 |
| materialize | 1.00x | 2.22x | 1.80x | 1.20x | 2.40x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.38x | 1.69x | 1.42x | 2.73x | 2.21x | 30/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.56x | 1.21x | 2.24x | 1.78x | 30/30 |
| fen_parse | 1.00x | 1.42x | 1.52x | 1.14x | 2.01x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 3.99x | 10.00x | 1.90x | 28.91x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.90x | 1.25x | 3.70x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.02x | 2.36x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.55x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.95x | 1.34x | 4.55x | 1.49x | 1.69x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 97.56x | 134.30x | 221.14x | 151.01x | 5/5 |
| chess960_materialize | 1.00x | 1.91x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.28x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.85x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.63x | 1.79x | 1.37x | 2.51x | 2.05x | 9/10 |
| corpus_materialize | 0.98x | 1.95x | 1.81x | 1.25x | 2.95x | 1.53x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.34x | 3.13x | 2.35x | 2.95x | 1.66x | 10/10 |
| replay_trusted_native | 0.98x | 1.17x | 2.12x | 2.28x | 3.92x | 4.77x | 10/10 |
| replay_checked_native | 1.00x | 3.00x | 1.14x | 6.40x | 7.32x | 8.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
