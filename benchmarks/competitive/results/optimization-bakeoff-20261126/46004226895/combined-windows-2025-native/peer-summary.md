| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.89x | 1.80x | 1.75x | 2.70x | 2.20x | 30/30 |
| materialize | 1.04x | 2.03x | 1.36x | 1.16x | 2.37x | 1.52x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.15x | 1.70x | 1.50x | 2.56x | 2.35x | 30/30 |
| perft_full_d3 | 1.34x | 1.98x | 2.06x | 1.70x | 2.77x | 2.45x | 30/30 |
| fen_parse | 0.94x | 1.33x | 1.50x | 1.37x | 2.10x | 1.46x | 30/30 |
| fen_write_native_ep | 0.98x | 3.88x | 7.71x | 1.54x | 30.36x | 1.96x | 30/30 |
| snapshot | 1.01x | 1.78x | 1.28x | 4.12x | 1.26x | 1.14x | 30/30 |
| san_parse_all | 0.98x | 1.90x | N/A | 4.65x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 9.46x | N/A | 3.37x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.95x | 1.33x | 8.30x | 1.54x | 1.77x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 89.24x | 133.32x | 202.04x | 146.67x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.92x | 1.60x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.21x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.67x | 1.89x | 1.38x | 2.52x | 2.27x | 10/10 |
| corpus_materialize | 1.01x | 1.79x | 1.39x | 1.20x | 2.84x | 1.53x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.89x | 2.41x | 2.00x | 2.31x | 1.52x | 10/10 |
| replay_trusted_native | 1.03x | 1.01x | 1.80x | 2.18x | 3.46x | 4.30x | 9/10 |
| replay_checked_native | 1.00x | 2.47x | 1.11x | 6.11x | 7.51x | 8.28x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
