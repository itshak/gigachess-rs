| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.05x | 1.84x | 1.74x | 2.54x | 2.22x | 30/30 |
| materialize | 1.03x | 2.05x | 1.40x | 1.35x | 2.42x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.27x | 1.71x | 1.45x | 2.50x | 2.17x | 30/30 |
| perft_full_d3 | 1.22x | 1.72x | 1.83x | 1.65x | 2.39x | 2.28x | 30/30 |
| fen_parse | 1.02x | 1.32x | 1.49x | 1.41x | 2.25x | 1.47x | 30/30 |
| fen_write_native_ep | 0.95x | 3.83x | 7.64x | 1.45x | 29.80x | 1.87x | 30/30 |
| snapshot | 0.99x | 1.83x | 1.05x | 4.19x | 1.00x | 1.00x | 21/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 5.65x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.11x | N/A | 3.90x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.33x | 1.43x | 9.54x | 1.49x | 1.69x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 88.94x | 164.75x | 234.65x | 172.79x | 5/5 |
| chess960_materialize | 1.03x | 1.65x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.90x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 22.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.78x | 2.00x | 1.48x | 2.66x | 2.58x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.25x | 1.20x | 2.00x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.76x | 2.18x | 1.80x | 2.17x | 1.41x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.59x | 2.11x | 2.61x | 2.94x | 9/10 |
| replay_checked_native | 1.01x | 2.47x | 1.09x | 8.65x | 8.25x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
