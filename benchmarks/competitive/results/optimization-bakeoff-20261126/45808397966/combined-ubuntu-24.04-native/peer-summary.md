| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.97x | 1.70x | 1.43x | 2.31x | 1.73x | 30/30 |
| materialize | 1.00x | 2.10x | 1.57x | 1.61x | 2.48x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.89x | 3.49x | 1.43x | 1.09x | 2.32x | 1.71x | 29/30 |
| perft_full_d3 | 0.97x | 1.45x | 1.69x | 1.62x | 2.11x | 1.83x | 30/30 |
| fen_parse | 0.94x | 1.40x | 1.24x | 1.06x | 1.42x | 1.16x | 29/30 |
| fen_write_native_ep | 1.02x | 4.57x | 9.93x | 2.04x | 28.37x | 2.58x | 30/30 |
| snapshot | 1.42x | 1.83x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.94x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.15x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.90x | 1.21x | 3.63x | 1.19x | 1.50x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.08x | 1.11x | 108.48x | 158.62x | 212.24x | 174.49x | 5/5 |
| chess960_materialize | 1.14x | 1.81x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.42x | 1.45x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.25x | 1.60x | 0.87x | 1.93x | 1.79x | 8/10 |
| corpus_materialize | 0.99x | 1.39x | 1.30x | 1.08x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 3.02x | 2.25x | 2.92x | 1.57x | 10/10 |
| replay_trusted_native | 1.00x | 0.81x | 1.47x | 1.50x | 2.08x | 2.41x | 9/10 |
| replay_checked_native | 1.01x | 2.10x | 1.15x | 8.86x | 8.56x | 10.19x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
