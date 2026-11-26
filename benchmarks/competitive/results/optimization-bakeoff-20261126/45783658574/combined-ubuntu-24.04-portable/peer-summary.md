| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.48x | 1.34x | 1.82x | 1.29x | 29/30 |
| materialize | 0.96x | 2.19x | 1.66x | 1.23x | 2.22x | 1.64x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.83x | 1.51x | 1.18x | 1.89x | 1.62x | 30/30 |
| perft_full_d3 | 1.02x | 1.26x | 1.49x | 1.14x | 1.78x | 1.63x | 29/30 |
| fen_parse | 1.00x | 1.70x | 1.60x | 1.27x | 2.08x | 1.46x | 30/30 |
| fen_write_native_ep | 0.99x | 1.88x | 6.40x | 1.19x | 18.18x | 1.39x | 30/30 |
| snapshot | 1.00x | 1.72x | 1.20x | 2.88x | 1.14x | 0.93x | 24/30 |
| san_parse_all | 0.99x | 2.26x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.38x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.89x | 1.19x | 4.35x | 1.10x | 1.13x | 25/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 73.70x | 104.48x | 165.37x | 112.14x | 5/5 |
| chess960_materialize | 1.01x | 1.76x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.15x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 21.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.11x | 1.76x | 1.23x | 1.98x | 1.83x | 10/10 |
| corpus_materialize | 1.00x | 1.93x | 1.79x | 1.31x | 2.89x | 1.71x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.06x | 2.93x | 2.22x | 2.78x | 1.59x | 10/10 |
| replay_trusted_native | 1.01x | 0.69x | 1.38x | 1.51x | 2.20x | 2.03x | 8/10 |
| replay_checked_native | 0.98x | 2.26x | 0.97x | 5.34x | 6.21x | 7.26x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
