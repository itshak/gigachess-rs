| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.80x | 2.18x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.87x | 1.57x | 1.43x | 2.02x | 2.07x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.61x | 1.34x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.30x | 1.25x | 1.72x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.32x | 1.59x | 29.14x | 2.41x | 30/30 |
| snapshot | 1.00x | 1.11x | 1.03x | 1.47x | 1.04x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.77x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 3.99x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.58x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.46x | 128.75x | 168.85x | 137.03x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.01x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.43x | 1.71x | 1.37x | 1.99x | 2.21x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.53x | 1.22x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.64x | 1.66x | 2.00x | 2.21x | 2.26x | 8/10 |
| replay_checked_native | 1.05x | 2.24x | 1.09x | 7.36x | 6.59x | 8.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
