| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.89x | 1.49x | 1.63x | 1.93x | 1.93x | 30/30 |
| materialize | 1.00x | 1.82x | 1.44x | 1.25x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.41x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 1.01x | 1.19x | 1.29x | 1.07x | 1.58x | 1.49x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.34x | 1.28x | 1.78x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.37x | 1.58x | 30.01x | 2.40x | 30/30 |
| snapshot | 0.92x | 1.11x | 1.05x | 1.45x | 1.05x | 0.95x | 24/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.63x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.27x | 1.51x | 4.17x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.30x | 76.18x | 128.83x | 171.51x | 138.52x | 5/5 |
| chess960_materialize | 1.04x | 1.58x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.65x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 18.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.81x | 3.87x | 1.50x | 1.20x | 1.76x | 1.93x | 10/10 |
| corpus_materialize | 0.96x | 1.50x | 1.44x | 1.15x | 1.80x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.93x | 2.08x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.05x | 0.55x | 1.43x | 1.74x | 1.93x | 1.98x | 8/10 |
| replay_checked_native | 1.04x | 2.09x | 1.02x | 6.82x | 6.10x | 7.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
