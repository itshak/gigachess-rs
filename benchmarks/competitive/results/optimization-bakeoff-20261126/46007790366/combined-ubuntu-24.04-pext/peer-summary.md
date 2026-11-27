| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 5.25x | 1.77x | 1.73x | 2.56x | 2.12x | 30/30 |
| materialize | 1.00x | 2.16x | 1.67x | 1.26x | 2.39x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.51x | 1.70x | 1.38x | 2.61x | 2.38x | 30/30 |
| perft_full_d3 | 1.56x | 1.83x | 2.21x | 1.64x | 2.78x | 2.65x | 30/30 |
| fen_parse | 0.90x | 1.24x | 1.43x | 1.12x | 1.83x | 1.19x | 29/30 |
| fen_write_native_ep | 1.01x | 4.02x | 9.00x | 1.54x | 25.20x | 1.89x | 30/30 |
| snapshot | 1.02x | 2.49x | 1.50x | 4.49x | 1.50x | 1.50x | 30/30 |
| san_parse_all | 1.03x | 2.02x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.08x | N/A | 2.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.90x | 1.42x | 4.06x | 1.36x | 1.90x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.18x | N/A | N/A | 5/6 |
| startpos | 1.04x | 1.25x | 85.92x | 153.72x | 213.01x | 160.44x | 5/5 |
| chess960_materialize | 1.02x | 1.73x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 3.01x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 34.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.26x | 1.22x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 27.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.59x | 1.91x | 1.28x | 2.48x | 2.27x | 9/10 |
| corpus_materialize | 1.07x | 1.78x | 1.64x | 1.21x | 2.30x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.93x | 2.16x | 2.79x | 2.26x | 2.80x | 1.62x | 10/10 |
| replay_trusted_native | 1.02x | 0.76x | 1.46x | 1.58x | 2.50x | 2.92x | 9/10 |
| replay_checked_native | 1.01x | 2.65x | 1.23x | 9.42x | 9.49x | 11.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
