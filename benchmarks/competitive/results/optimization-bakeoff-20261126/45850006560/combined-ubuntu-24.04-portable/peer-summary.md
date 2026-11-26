| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.93x | 1.73x | 1.40x | 1.87x | 1.34x | 29/30 |
| materialize | 0.97x | 2.38x | 1.88x | 1.85x | 2.74x | 1.95x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.93x | 1.56x | 1.16x | 1.93x | 1.73x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.60x | 1.40x | 1.90x | 1.63x | 30/30 |
| fen_parse | 0.95x | 1.61x | 1.37x | 1.09x | 1.41x | 1.12x | 29/30 |
| fen_write_native_ep | 0.99x | 3.06x | 10.24x | 1.99x | 29.01x | 2.35x | 30/30 |
| snapshot | 0.80x | 2.31x | 2.05x | 3.75x | 1.00x | 1.00x | 25/30 |
| san_parse_all | 0.99x | 1.87x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.21x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.01x | 1.34x | 3.91x | 1.77x | 1.46x | 30/30 |
| make_unmake_full | 0.91x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 57.43x | 104.99x | 121.08x | 103.88x | 5/5 |
| chess960_materialize | 0.93x | 1.70x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.01x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 37.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 19.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.10x | 2.08x | 1.14x | 2.10x | 2.82x | 10/10 |
| corpus_materialize | 0.98x | 1.47x | 1.36x | 1.11x | 1.77x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.93x | 3.06x | 2.22x | 2.99x | 1.54x | 10/10 |
| replay_trusted_native | 0.91x | 0.80x | 1.54x | 1.50x | 2.06x | 1.89x | 8/10 |
| replay_checked_native | 0.96x | 1.84x | 1.03x | 7.92x | 7.77x | 8.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
