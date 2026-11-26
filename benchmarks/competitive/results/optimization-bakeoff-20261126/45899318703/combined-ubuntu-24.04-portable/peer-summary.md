| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.16x | 1.62x | 1.46x | 2.04x | 1.41x | 30/30 |
| materialize | 1.01x | 2.44x | 1.93x | 1.37x | 2.61x | 1.75x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.07x | 1.63x | 1.29x | 2.05x | 1.76x | 30/30 |
| perft_full_d3 | 1.01x | 1.42x | 1.68x | 1.28x | 1.97x | 1.84x | 30/30 |
| fen_parse | 1.00x | 1.46x | 1.35x | 1.06x | 1.73x | 1.31x | 28/30 |
| fen_write_native_ep | 0.99x | 2.62x | 9.39x | 1.72x | 25.86x | 1.90x | 30/30 |
| snapshot | 1.01x | 1.68x | 1.23x | 2.83x | 1.13x | 0.99x | 25/30 |
| san_parse_all | 1.00x | 2.23x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.38x | N/A | 2.29x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.44x | 1.50x | 5.66x | 1.42x | 1.47x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.38x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 73.19x | 104.83x | 161.18x | 119.38x | 5/5 |
| chess960_materialize | 1.04x | 1.95x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.40x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 21.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 20.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.21x | 1.77x | 1.24x | 1.93x | 1.85x | 10/10 |
| corpus_materialize | 1.03x | 2.07x | 1.84x | 1.35x | 3.03x | 1.71x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.08x | 3.02x | 2.24x | 2.87x | 1.59x | 10/10 |
| replay_trusted_native | 0.99x | 1.07x | 2.17x | 2.32x | 3.44x | 3.12x | 10/10 |
| replay_checked_native | 1.01x | 2.61x | 1.16x | 6.23x | 7.01x | 8.11x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
