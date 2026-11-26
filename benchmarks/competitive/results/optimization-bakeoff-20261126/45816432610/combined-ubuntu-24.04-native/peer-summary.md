| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.98x | 1.73x | 1.46x | 2.23x | 1.76x | 30/30 |
| materialize | 1.05x | 2.11x | 1.75x | 1.64x | 2.71x | 1.83x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.48x | 1.43x | 1.09x | 2.25x | 1.72x | 29/30 |
| perft_full_d3 | 1.03x | 1.46x | 1.69x | 1.64x | 2.44x | 1.82x | 30/30 |
| fen_parse | 0.91x | 1.15x | 1.26x | 1.05x | 1.44x | 1.13x | 29/30 |
| fen_write_native_ep | 1.01x | 4.62x | 10.64x | 1.91x | 30.42x | 2.20x | 30/30 |
| snapshot | 1.66x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.93x | N/A | 4.31x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.01x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.89x | 1.19x | 3.62x | 1.19x | 1.50x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 1.34x | 1.12x | 88.33x | 163.01x | 206.00x | 172.56x | 5/5 |
| chess960_materialize | 1.09x | 1.82x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.40x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 18.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.64x | 4.15x | 1.56x | 0.86x | 1.89x | 1.73x | 8/10 |
| corpus_materialize | 0.99x | 1.39x | 1.30x | 1.08x | 1.71x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.22x | 3.06x | 2.23x | 2.96x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.79x | 1.39x | 1.45x | 2.00x | 2.34x | 8/10 |
| replay_checked_native | 1.01x | 2.08x | 1.10x | 8.79x | 8.40x | 10.10x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
