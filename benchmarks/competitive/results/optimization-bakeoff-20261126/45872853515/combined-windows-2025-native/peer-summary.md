| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.80x | 1.75x | 1.69x | 2.73x | 2.13x | 30/30 |
| materialize | 1.00x | 2.02x | 1.35x | 1.14x | 2.36x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.03x | 3.99x | 1.63x | 1.44x | 2.45x | 2.28x | 30/30 |
| perft_full_d3 | 1.29x | 1.94x | 1.99x | 1.61x | 2.72x | 2.39x | 30/30 |
| fen_parse | 0.97x | 1.41x | 1.46x | 1.38x | 2.17x | 1.46x | 30/30 |
| fen_write_native_ep | 0.98x | 3.94x | 7.62x | 1.58x | 30.67x | 1.97x | 30/30 |
| snapshot | 1.00x | 1.79x | 1.26x | 4.13x | 1.26x | 1.17x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.76x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.38x | N/A | 3.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.96x | 1.31x | 8.28x | 1.55x | 2.11x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 87.20x | 132.21x | 204.70x | 146.40x | 5/5 |
| chess960_materialize | 1.02x | 1.65x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.89x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.54x | 1.78x | 1.30x | 2.34x | 2.16x | 9/10 |
| corpus_materialize | 1.01x | 1.76x | 1.39x | 1.19x | 2.87x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.85x | 2.37x | 1.96x | 2.24x | 1.47x | 10/10 |
| replay_trusted_native | 1.08x | 1.04x | 1.80x | 2.27x | 3.60x | 4.45x | 9/10 |
| replay_checked_native | 0.95x | 2.45x | 1.05x | 6.03x | 7.80x | 8.50x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
