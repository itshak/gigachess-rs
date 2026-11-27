| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.49x | 1.90x | 1.64x | 2.29x | 2.06x | 30/30 |
| materialize | 0.99x | 2.00x | 1.54x | 1.61x | 2.40x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.00x | 1.63x | 1.24x | 2.26x | 1.97x | 29/30 |
| perft_full_d3 | 1.26x | 1.72x | 1.98x | 1.84x | 2.43x | 2.14x | 30/30 |
| fen_parse | 0.90x | 1.19x | 1.38x | 1.06x | 1.48x | 1.14x | 30/30 |
| fen_write_native_ep | 0.98x | 5.31x | 10.26x | 2.08x | 28.76x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 1.84x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.37x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.14x | 1.36x | 4.17x | 1.34x | 1.70x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 90.53x | 155.30x | 198.82x | 162.48x | 5/5 |
| chess960_materialize | 1.01x | 1.58x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.57x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 25.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.42x | 6.48x | 2.53x | 1.29x | 2.88x | 2.83x | 10/10 |
| corpus_materialize | 1.02x | 1.39x | 1.31x | 1.06x | 1.67x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.36x | 2.99x | 2.21x | 2.89x | 1.56x | 10/10 |
| replay_trusted_native | 1.01x | 0.85x | 1.50x | 1.51x | 2.13x | 2.46x | 9/10 |
| replay_checked_native | 1.00x | 2.33x | 1.12x | 9.13x | 8.96x | 10.42x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
