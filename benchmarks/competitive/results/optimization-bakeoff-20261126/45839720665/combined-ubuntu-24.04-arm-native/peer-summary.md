| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.67x | 1.81x | 2.17x | 2.17x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.57x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.01x | 0.93x | 1.02x | 0.84x | 1.24x | 1.17x | 17/30 |
| fen_parse | 1.00x | 1.23x | 1.32x | 1.26x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.49x | 9.43x | 1.58x | 30.04x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.08x | 1.05x | 1.47x | 1.04x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.76x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.00x | 1.20x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.20x | 128.13x | 172.82x | 140.36x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.96x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.89x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.47x | 1.70x | 1.37x | 1.99x | 2.21x | 10/10 |
| corpus_materialize | 1.01x | 1.62x | 1.53x | 1.23x | 1.91x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.09x | 2.85x | 1.45x | 10/10 |
| replay_trusted_native | 1.02x | 0.62x | 1.66x | 2.00x | 2.25x | 2.27x | 8/10 |
| replay_checked_native | 1.10x | 2.36x | 1.15x | 7.67x | 6.93x | 8.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
