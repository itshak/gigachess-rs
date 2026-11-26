| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.26x | 1.79x | 1.54x | 2.16x | 1.92x | 30/30 |
| materialize | 0.98x | 1.98x | 1.61x | 1.62x | 2.32x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.53x | 1.17x | 2.13x | 1.85x | 29/30 |
| perft_full_d3 | 1.05x | 1.36x | 1.61x | 1.47x | 1.90x | 1.70x | 30/30 |
| fen_parse | 1.02x | 1.30x | 1.45x | 1.17x | 1.62x | 1.30x | 30/30 |
| fen_write_native_ep | 1.01x | 5.04x | 10.64x | 1.96x | 31.34x | 2.64x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.01x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 0.93x | 1.70x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.55x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.04x | 1.31x | 4.03x | 1.22x | 1.71x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.25x | 102.18x | 174.86x | 233.41x | 191.83x | 5/5 |
| chess960_materialize | 0.94x | 1.50x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.52x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 37.84x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.77x | 23.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.66x | 1.83x | 0.93x | 2.10x | 2.03x | 8/10 |
| corpus_materialize | 0.99x | 1.37x | 1.28x | 1.04x | 1.67x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.99x | 2.23x | 2.86x | 1.57x | 10/10 |
| replay_trusted_native | 1.02x | 0.88x | 1.59x | 1.55x | 2.16x | 2.78x | 9/10 |
| replay_checked_native | 1.00x | 2.15x | 1.14x | 8.63x | 8.36x | 10.03x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
