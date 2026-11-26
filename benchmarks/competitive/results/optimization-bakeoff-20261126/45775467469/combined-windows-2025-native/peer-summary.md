| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.00x | 1.68x | 1.46x | 2.03x | 1.79x | 30/30 |
| materialize | 1.02x | 1.96x | 1.38x | 1.52x | 2.31x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.77x | 1.53x | 1.16x | 2.10x | 1.86x | 30/30 |
| perft_full_d3 | 1.66x | 1.42x | 1.56x | 1.48x | 2.00x | 1.69x | 30/30 |
| fen_parse | 1.10x | 1.20x | 1.40x | 1.23x | 1.76x | 1.45x | 30/30 |
| fen_write_native_ep | 1.01x | 3.20x | 5.89x | 1.32x | 22.90x | 1.65x | 30/30 |
| snapshot | 2.65x | 1.52x | 1.09x | 2.64x | 1.12x | 1.01x | 28/30 |
| san_parse_all | 1.01x | 1.59x | N/A | 4.64x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.35x | N/A | 3.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.47x | 0.97x | 7.42x | 0.96x | 1.21x | 24/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.00x | N/A | N/A | 4/6 |
| startpos | 0.95x | 1.21x | 62.35x | 121.65x | 171.14x | 136.30x | 5/5 |
| chess960_materialize | 1.02x | 1.46x | 1.31x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.31x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.32x | 41.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 24.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.05x | 6.45x | 2.51x | 1.31x | 2.94x | 2.82x | 10/10 |
| corpus_materialize | 1.02x | 1.43x | 1.26x | 1.10x | 1.75x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.15x | 1.38x | 1.18x | 1.37x | 0.98x | 9/10 |
| replay_trusted_native | 0.97x | 0.58x | 1.03x | 1.05x | 1.42x | 1.63x | 6/10 |
| replay_checked_native | 1.08x | 1.86x | 0.96x | 7.38x | 7.16x | 8.51x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
