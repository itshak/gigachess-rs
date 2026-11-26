| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.23x | 1.63x | 1.48x | 2.06x | 1.44x | 30/30 |
| materialize | 1.00x | 2.22x | 1.88x | 1.34x | 2.52x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.12x | 1.68x | 1.31x | 2.11x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.38x | 1.58x | 1.28x | 1.90x | 1.74x | 30/30 |
| fen_parse | 1.05x | 1.57x | 1.63x | 1.37x | 2.38x | 1.36x | 30/30 |
| fen_write_native_ep | 0.97x | 2.74x | 9.31x | 1.65x | 25.85x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.10x | 3.03x | 0.99x | 1.06x | 28/30 |
| san_parse_all | 1.00x | 2.03x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.36x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.19x | 1.52x | 4.50x | 1.40x | 1.44x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.39x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 66.24x | 122.06x | 185.01x | 112.56x | 5/5 |
| chess960_materialize | 0.98x | 1.62x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.28x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 23.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 18.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.93x | 1.93x | 1.29x | 2.13x | 2.58x | 10/10 |
| corpus_materialize | 1.00x | 1.65x | 1.65x | 1.20x | 2.10x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.21x | 3.29x | 2.46x | 3.08x | 1.75x | 10/10 |
| replay_trusted_native | 0.99x | 0.89x | 1.64x | 1.95x | 2.54x | 2.36x | 9/10 |
| replay_checked_native | 0.99x | 2.10x | 1.03x | 7.26x | 7.38x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
