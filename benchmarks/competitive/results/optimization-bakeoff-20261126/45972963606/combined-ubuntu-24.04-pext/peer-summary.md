| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.69x | 1.96x | 1.70x | 2.67x | 2.00x | 30/30 |
| materialize | 1.01x | 2.29x | 1.69x | 1.78x | 2.81x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.02x | 1.59x | 1.25x | 2.49x | 1.95x | 29/30 |
| perft_full_d3 | 1.22x | 1.65x | 1.92x | 1.85x | 2.46x | 2.00x | 30/30 |
| fen_parse | 1.00x | 1.48x | 1.36x | 1.16x | 1.49x | 1.14x | 30/30 |
| fen_write_native_ep | 1.00x | 4.93x | 10.49x | 2.05x | 27.70x | 2.57x | 30/30 |
| snapshot | 1.00x | 2.01x | 1.00x | 4.68x | 1.00x | 1.33x | 25/30 |
| san_parse_all | 0.93x | 1.73x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.28x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 2.16x | 1.32x | 3.95x | 1.41x | 1.61x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.00x | 78.46x | 143.86x | 174.10x | 142.33x | 4/5 |
| chess960_materialize | 1.11x | 1.75x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.70x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.77x | 18.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 6.34x | 2.36x | 1.30x | 2.87x | 2.67x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.32x | 1.09x | 1.79x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.34x | 3.06x | 2.25x | 2.99x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.60x | 1.58x | 2.17x | 2.49x | 9/10 |
| replay_checked_native | 0.96x | 2.14x | 1.16x | 8.82x | 8.64x | 10.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
