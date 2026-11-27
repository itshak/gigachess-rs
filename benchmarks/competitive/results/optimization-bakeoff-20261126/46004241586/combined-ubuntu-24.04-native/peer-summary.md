| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 4.47x | 1.92x | 1.62x | 2.51x | 1.98x | 30/30 |
| materialize | 0.95x | 2.10x | 1.59x | 1.74x | 2.62x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.95x | 1.63x | 1.24x | 2.44x | 1.96x | 29/30 |
| perft_full_d3 | 1.23x | 1.67x | 1.93x | 1.82x | 2.42x | 2.04x | 30/30 |
| fen_parse | 0.99x | 1.40x | 1.33x | 1.19x | 1.50x | 1.25x | 30/30 |
| fen_write_native_ep | 1.02x | 4.71x | 10.06x | 1.87x | 29.71x | 2.53x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.33x | 1.15x | 1.34x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.03x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.12x | 1.39x | 4.11x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.12x | 88.23x | 168.11x | 191.25x | 170.24x | 5/5 |
| chess960_materialize | 1.01x | 1.76x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.52x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 36.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.21x | 6.32x | 2.38x | 1.31x | 2.91x | 2.68x | 10/10 |
| corpus_materialize | 1.01x | 1.39x | 1.28x | 1.05x | 1.72x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.21x | 2.93x | 2.21x | 2.90x | 1.54x | 10/10 |
| replay_trusted_native | 1.02x | 0.89x | 1.61x | 1.57x | 2.18x | 2.58x | 9/10 |
| replay_checked_native | 0.98x | 2.17x | 1.16x | 8.85x | 8.57x | 10.24x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
