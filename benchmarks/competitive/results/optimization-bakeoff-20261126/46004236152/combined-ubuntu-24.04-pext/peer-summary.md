| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.69x | 1.96x | 1.71x | 2.63x | 1.99x | 30/30 |
| materialize | 1.08x | 2.30x | 1.71x | 1.80x | 2.85x | 1.94x | 30/30 |
| perft_bulk_d4 | 1.07x | 4.13x | 1.82x | 1.28x | 2.50x | 1.99x | 29/30 |
| perft_full_d3 | 1.24x | 2.18x | 2.51x | 1.88x | 2.44x | 2.02x | 30/30 |
| fen_parse | 1.00x | 1.39x | 1.32x | 1.16x | 1.45x | 1.24x | 30/30 |
| fen_write_native_ep | 0.97x | 4.92x | 9.85x | 1.82x | 28.28x | 2.44x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.11x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.16x | 1.35x | 4.08x | 1.35x | 1.70x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.13x | 88.42x | 167.79x | 190.63x | 172.56x | 5/5 |
| chess960_materialize | 1.07x | 1.76x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.54x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 23.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 6.20x | 2.31x | 1.28x | 2.83x | 2.54x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.34x | 1.09x | 1.78x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.32x | 3.06x | 2.29x | 3.02x | 1.57x | 10/10 |
| replay_trusted_native | 1.02x | 0.93x | 1.64x | 1.63x | 2.27x | 2.64x | 9/10 |
| replay_checked_native | 0.96x | 2.13x | 1.11x | 8.87x | 8.67x | 9.95x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
