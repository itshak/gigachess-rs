| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.50x | 1.36x | 1.90x | 1.37x | 30/30 |
| materialize | 1.00x | 2.29x | 1.91x | 1.32x | 2.49x | 1.66x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.86x | 1.50x | 1.18x | 1.91x | 1.62x | 30/30 |
| perft_full_d3 | 0.92x | 1.17x | 1.42x | 1.08x | 1.68x | 1.53x | 29/30 |
| fen_parse | 0.94x | 1.52x | 1.56x | 1.19x | 2.18x | 1.40x | 30/30 |
| fen_write_native_ep | 1.00x | 2.64x | 9.04x | 1.78x | 25.73x | 2.04x | 30/30 |
| snapshot | 1.00x | 1.59x | 1.20x | 2.92x | 1.14x | 1.07x | 30/30 |
| san_parse_all | 1.02x | 2.28x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.87x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.85x | 1.18x | 4.37x | 1.10x | 1.15x | 25/30 |
| make_unmake_full | 0.86x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 72.74x | 107.53x | 168.31x | 117.45x | 5/5 |
| chess960_materialize | 0.93x | 1.78x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.20x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 16.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 2.95x | 1.67x | 1.16x | 1.80x | 1.72x | 10/10 |
| corpus_materialize | 1.03x | 1.93x | 1.85x | 1.31x | 2.88x | 1.67x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.03x | 2.89x | 2.18x | 2.73x | 1.55x | 10/10 |
| replay_trusted_native | 0.90x | 0.73x | 1.46x | 1.60x | 2.42x | 2.15x | 8/10 |
| replay_checked_native | 0.98x | 2.41x | 1.00x | 5.38x | 5.84x | 6.81x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
