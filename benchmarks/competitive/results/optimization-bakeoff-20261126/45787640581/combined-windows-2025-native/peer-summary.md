| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.35x | 1.59x | 1.53x | 2.38x | 1.93x | 30/30 |
| materialize | 1.02x | 1.94x | 1.31x | 1.09x | 2.17x | 1.39x | 28/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.53x | 1.35x | 2.35x | 2.12x | 30/30 |
| perft_full_d3 | 0.98x | 1.23x | 1.28x | 1.06x | 1.73x | 1.56x | 27/30 |
| fen_parse | 1.07x | 1.29x | 1.40x | 1.34x | 2.34x | 1.45x | 30/30 |
| fen_write_native_ep | 1.00x | 4.04x | 7.71x | 1.62x | 31.32x | 2.05x | 30/30 |
| snapshot | 0.72x | 1.21x | 0.87x | 1.99x | 0.88x | 0.81x | 12/30 |
| san_parse_all | 0.99x | 1.87x | N/A | 4.68x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.53x | N/A | 3.43x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.49x | 1.03x | 6.05x | 1.15x | 1.30x | 27/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.18x | 91.37x | 168.18x | 246.98x | 182.61x | 5/5 |
| chess960_materialize | 1.04x | 1.56x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.72x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.91x | 17.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 3.76x | 1.53x | 1.14x | 2.04x | 1.90x | 9/10 |
| corpus_materialize | 0.95x | 1.68x | 1.35x | 1.14x | 2.67x | 1.45x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.90x | 2.49x | 1.99x | 2.38x | 1.56x | 10/10 |
| replay_trusted_native | 0.95x | 0.73x | 1.27x | 1.55x | 2.60x | 3.23x | 8/10 |
| replay_checked_native | 0.92x | 2.11x | 0.86x | 5.33x | 6.54x | 7.03x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
