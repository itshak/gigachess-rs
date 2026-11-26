| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.73x | 1.44x | 2.36x | 1.75x | 30/30 |
| materialize | 1.00x | 2.10x | 1.59x | 1.65x | 2.53x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.89x | 1.59x | 1.22x | 2.46x | 1.91x | 30/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.77x | 1.64x | 2.09x | 1.78x | 30/30 |
| fen_parse | 1.07x | 1.30x | 1.33x | 1.01x | 1.42x | 1.21x | 26/30 |
| fen_write_native_ep | 0.99x | 3.16x | 6.43x | 1.37x | 18.32x | 1.70x | 30/30 |
| snapshot | 1.00x | 1.41x | 1.16x | 3.23x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 0.93x | 1.94x | N/A | 4.08x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.16x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.73x | 1.09x | 3.23x | 1.12x | 1.79x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.21x | 84.71x | 142.13x | 177.65x | 160.39x | 5/5 |
| chess960_materialize | 0.99x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.27x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.08x | 21.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.45x | 2.42x | 1.33x | 2.93x | 2.66x | 10/10 |
| corpus_materialize | 1.01x | 1.45x | 1.32x | 1.08x | 1.75x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.03x | 2.20x | 2.93x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.72x | 1.32x | 1.25x | 1.79x | 2.09x | 8/10 |
| replay_checked_native | 1.02x | 2.05x | 1.09x | 8.57x | 8.27x | 9.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
