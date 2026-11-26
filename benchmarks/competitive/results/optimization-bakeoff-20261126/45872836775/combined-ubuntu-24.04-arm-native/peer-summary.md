| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.41x | 1.67x | 1.82x | 2.17x | 2.16x | 30/30 |
| materialize | 1.01x | 1.96x | 1.52x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.85x | 1.57x | 1.44x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.60x | 1.34x | 1.98x | 1.86x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.30x | 1.26x | 1.72x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.28x | 1.57x | 29.71x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.09x | 1.04x | 1.47x | 1.04x | 0.95x | 24/30 |
| san_parse_all | 0.99x | 1.86x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.44x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.08x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.59x | 128.52x | 172.44x | 138.25x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.93x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.64x | 1.70x | 1.34x | 2.01x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.68x | 1.53x | 1.22x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.90x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.71x | 1.99x | 2.24x | 2.28x | 8/10 |
| replay_checked_native | 1.01x | 2.40x | 1.14x | 7.61x | 6.94x | 8.89x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
