| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.50x | 1.42x | 1.23x | 1.76x | 1.21x | 28/30 |
| materialize | 1.09x | 2.17x | 1.67x | 1.70x | 2.57x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.12x | 2.69x | 1.42x | 1.08x | 1.91x | 1.63x | 30/30 |
| perft_full_d3 | 1.43x | 0.92x | 1.10x | 0.96x | 1.29x | 1.11x | 20/30 |
| fen_parse | 1.08x | 1.46x | 1.36x | 1.05x | 1.41x | 1.16x | 29/30 |
| fen_write_native_ep | 1.01x | 2.07x | 6.56x | 1.27x | 18.17x | 1.68x | 30/30 |
| snapshot | 1.90x | 1.46x | 1.12x | 2.91x | 1.12x | 1.00x | 28/30 |
| san_parse_all | 2.23x | 1.76x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 2.53x | 9.02x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 2.25x | 1.57x | 1.03x | 2.98x | 0.95x | 1.11x | 24/30 |
| make_unmake_full | 1.29x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 1.72x | 1.29x | 57.70x | 104.78x | 125.09x | 108.71x | 5/5 |
| chess960_materialize | 1.14x | 1.58x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.19x | 1.69x | 1.18x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 38.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.45x | 20.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.44x | 2.22x | 1.23x | 2.25x | 3.10x | 10/10 |
| corpus_materialize | 1.01x | 1.50x | 1.41x | 1.16x | 1.86x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.19x | 0.95x | 1.44x | 1.08x | 1.42x | 0.76x | 6/10 |
| replay_trusted_native | 1.20x | 0.60x | 1.11x | 1.12x | 1.53x | 1.41x | 7/10 |
| replay_checked_native | 1.25x | 1.71x | 0.97x | 7.34x | 7.24x | 8.06x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
