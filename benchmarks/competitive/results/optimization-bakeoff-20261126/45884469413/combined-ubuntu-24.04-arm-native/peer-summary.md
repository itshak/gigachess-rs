| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.46x | 1.70x | 1.86x | 2.21x | 2.19x | 30/30 |
| materialize | 1.01x | 1.97x | 1.56x | 1.34x | 2.19x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.90x | 1.59x | 1.45x | 2.04x | 2.08x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.60x | 1.33x | 1.97x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.23x | 1.33x | 1.28x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.42x | 9.28x | 1.57x | 29.84x | 2.40x | 30/30 |
| snapshot | 1.00x | 1.09x | 1.02x | 1.47x | 1.04x | 0.95x | 21/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.46x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.37x | 125.96x | 172.41x | 139.63x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.96x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.47x | 1.72x | 1.37x | 2.01x | 2.22x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.53x | 1.23x | 1.91x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.64x | 1.98x | 2.21x | 2.25x | 8/10 |
| replay_checked_native | 1.02x | 2.33x | 1.15x | 7.64x | 6.87x | 8.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
