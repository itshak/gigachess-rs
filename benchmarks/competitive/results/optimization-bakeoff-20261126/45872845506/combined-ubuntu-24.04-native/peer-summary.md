| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.25x | 1.82x | 1.54x | 2.31x | 1.87x | 30/30 |
| materialize | 0.96x | 2.11x | 1.59x | 1.70x | 2.58x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.76x | 1.54x | 1.17x | 2.27x | 1.84x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.62x | 1.50x | 1.93x | 1.70x | 30/30 |
| fen_parse | 0.98x | 1.47x | 1.32x | 1.18x | 1.45x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 5.01x | 10.37x | 2.06x | 30.15x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.95x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.29x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.15x | 1.38x | 4.16x | 1.28x | 1.77x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.24x | 99.16x | 174.65x | 213.93x | 176.14x | 5/5 |
| chess960_materialize | 0.90x | 1.58x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.26x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.25x | 1.99x | 1.10x | 2.38x | 2.21x | 10/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.05x | 1.70x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 3.06x | 2.24x | 2.99x | 1.54x | 10/10 |
| replay_trusted_native | 0.96x | 0.85x | 1.50x | 1.52x | 2.07x | 2.45x | 9/10 |
| replay_checked_native | 0.98x | 2.16x | 1.17x | 8.91x | 8.52x | 10.22x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
