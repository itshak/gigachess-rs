| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.51x | 1.30x | 1.81x | 1.29x | 29/30 |
| materialize | 0.94x | 2.04x | 1.64x | 1.23x | 2.34x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.02x | 1.54x | 1.16x | 1.93x | 1.71x | 30/30 |
| perft_full_d3 | 0.95x | 1.39x | 1.72x | 1.31x | 1.93x | 1.74x | 30/30 |
| fen_parse | 1.00x | 1.65x | 1.57x | 1.24x | 1.94x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 2.60x | 8.55x | 1.55x | 25.82x | 1.98x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.16x | 2.64x | 1.16x | 1.00x | 29/30 |
| san_parse_all | 0.97x | 1.80x | N/A | 3.73x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 10.46x | N/A | 2.26x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.69x | 1.12x | 3.45x | 0.98x | 1.02x | 23/30 |
| make_unmake_full | 1.39x | N/A | N/A | 1.15x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.28x | 59.07x | 82.22x | 115.50x | 86.49x | 5/5 |
| chess960_materialize | 0.98x | 1.53x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.00x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 32.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 20.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.43x | 1.83x | 1.12x | 2.00x | 2.89x | 10/10 |
| corpus_materialize | 1.01x | 1.82x | 1.67x | 1.26x | 2.38x | 1.45x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.01x | 2.85x | 2.24x | 2.88x | 1.59x | 10/10 |
| replay_trusted_native | 0.97x | 0.56x | 1.31x | 1.31x | 1.75x | 1.60x | 7/10 |
| replay_checked_native | 1.09x | 1.85x | 1.06x | 7.53x | 7.17x | 8.41x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
