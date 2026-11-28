| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.68x | 1.78x | 1.69x | 2.62x | 2.06x | 30/30 |
| materialize | 1.01x | 2.02x | 1.74x | 1.19x | 2.29x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.98x | 1.65x | 1.42x | 2.46x | 2.01x | 30/30 |
| perft_full_d3 | 1.32x | 1.67x | 2.12x | 1.67x | 2.65x | 2.52x | 30/30 |
| fen_parse | 1.02x | 1.26x | 1.44x | 1.19x | 2.01x | 1.33x | 30/30 |
| fen_write_native_ep | 1.01x | 4.24x | 10.31x | 1.85x | 28.43x | 2.12x | 30/30 |
| snapshot | 1.01x | 1.85x | 1.25x | 3.32x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.00x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.48x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.10x | 1.32x | 5.53x | 1.43x | 1.59x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 95.51x | 147.22x | 222.49x | 160.83x | 5/5 |
| chess960_materialize | 0.99x | 1.70x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.78x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 19.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.18x | 1.18x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 18.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.57x | 1.90x | 1.40x | 2.49x | 2.19x | 10/10 |
| corpus_materialize | 1.02x | 1.78x | 1.76x | 1.22x | 2.88x | 1.56x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.45x | 3.24x | 2.46x | 3.11x | 1.82x | 10/10 |
| replay_trusted_native | 1.02x | 1.13x | 2.04x | 2.37x | 3.54x | 4.09x | 10/10 |
| replay_checked_native | 1.02x | 2.43x | 1.10x | 6.19x | 7.97x | 8.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
