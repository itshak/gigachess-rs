| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.93x | 1.67x | 1.65x | 2.46x | 1.99x | 30/30 |
| materialize | 1.01x | 2.17x | 1.68x | 1.23x | 2.49x | 1.53x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.24x | 1.59x | 1.28x | 2.44x | 2.23x | 30/30 |
| perft_full_d3 | 1.02x | 1.21x | 1.43x | 1.11x | 1.85x | 1.77x | 29/30 |
| fen_parse | 1.01x | 1.30x | 1.43x | 1.20x | 1.94x | 1.28x | 29/30 |
| fen_write_native_ep | 0.99x | 4.20x | 8.97x | 1.54x | 25.70x | 1.98x | 30/30 |
| snapshot | 1.05x | 2.49x | 1.49x | 4.49x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 1.00x | 1.81x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.70x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.86x | 1.75x | 4.18x | 1.57x | 2.12x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.24x | 83.00x | 143.68x | 211.41x | 149.21x | 5/5 |
| chess960_materialize | 1.03x | 1.83x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.94x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 36.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 27.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.93x | 1.84x | 1.17x | 2.32x | 2.10x | 9/10 |
| corpus_materialize | 0.99x | 1.69x | 1.53x | 1.13x | 2.23x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 3.00x | 2.36x | 3.07x | 1.65x | 10/10 |
| replay_trusted_native | 0.99x | 0.70x | 1.37x | 1.49x | 2.34x | 2.77x | 8/10 |
| replay_checked_native | 1.00x | 2.50x | 1.10x | 8.48x | 8.59x | 10.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
