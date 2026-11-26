| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.52x | 1.88x | 1.64x | 2.91x | 1.91x | 30/30 |
| materialize | 0.93x | 2.11x | 1.55x | 1.73x | 2.59x | 1.96x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.86x | 1.53x | 1.20x | 2.53x | 1.87x | 29/30 |
| perft_full_d3 | 1.23x | 1.65x | 1.88x | 1.85x | 2.46x | 2.03x | 30/30 |
| fen_parse | 0.94x | 1.40x | 1.31x | 1.13x | 1.51x | 1.20x | 29/30 |
| fen_write_native_ep | 0.97x | 4.82x | 10.37x | 1.98x | 29.49x | 2.43x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 22/30 |
| san_parse_all | 1.02x | 2.05x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.05x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.08x | 1.31x | 3.95x | 1.39x | 1.60x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.10x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.00x | 78.56x | 144.49x | 185.09x | 154.57x | 4/5 |
| chess960_materialize | 0.94x | 1.64x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 23.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.09x | 1.90x | 1.06x | 2.37x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.41x | 1.29x | 1.07x | 1.72x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.11x | 2.28x | 3.02x | 1.53x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.53x | 1.58x | 2.19x | 2.54x | 9/10 |
| replay_checked_native | 1.00x | 2.19x | 1.11x | 9.12x | 8.84x | 10.41x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
