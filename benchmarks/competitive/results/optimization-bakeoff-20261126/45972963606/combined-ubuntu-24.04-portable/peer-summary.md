| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.39x | 1.78x | 1.56x | 2.28x | 1.51x | 30/30 |
| materialize | 1.01x | 2.50x | 1.88x | 1.37x | 2.54x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.24x | 1.72x | 1.36x | 2.25x | 1.84x | 30/30 |
| perft_full_d3 | 1.23x | 1.75x | 2.07x | 1.60x | 2.42x | 2.27x | 30/30 |
| fen_parse | 0.85x | 1.50x | 1.38x | 1.13x | 1.71x | 1.16x | 30/30 |
| fen_write_native_ep | 1.03x | 2.66x | 8.91x | 1.74x | 26.13x | 1.99x | 30/30 |
| snapshot | 1.00x | 1.94x | 1.65x | 2.84x | 1.13x | 0.97x | 24/30 |
| san_parse_all | 1.00x | 2.29x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.99x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.40x | 1.47x | 5.69x | 1.45x | 1.46x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 80.29x | 110.60x | 157.10x | 105.49x | 5/5 |
| chess960_materialize | 1.01x | 1.93x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.40x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 3.43x | 1.90x | 1.33x | 2.15x | 1.99x | 10/10 |
| corpus_materialize | 1.00x | 2.05x | 1.87x | 1.37x | 3.06x | 1.75x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.06x | 2.98x | 2.21x | 2.89x | 1.59x | 10/10 |
| replay_trusted_native | 1.00x | 1.09x | 2.19x | 2.39x | 3.47x | 3.16x | 10/10 |
| replay_checked_native | 1.02x | 2.76x | 1.18x | 6.37x | 6.79x | 8.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
