| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.49x | 1.93x | 1.64x | 2.56x | 2.00x | 30/30 |
| materialize | 1.01x | 2.21x | 1.70x | 1.73x | 2.64x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.96x | 1.63x | 1.24x | 2.46x | 1.97x | 29/30 |
| perft_full_d3 | 1.24x | 1.66x | 1.93x | 1.88x | 2.47x | 2.09x | 30/30 |
| fen_parse | 0.99x | 1.40x | 1.58x | 1.15x | 1.48x | 1.26x | 30/30 |
| fen_write_native_ep | 1.00x | 4.68x | 10.08x | 1.91x | 28.44x | 2.50x | 30/30 |
| snapshot | 1.00x | 1.99x | 1.00x | 4.64x | 1.00x | 1.32x | 24/30 |
| san_parse_all | 0.90x | 1.71x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 8.96x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.36x | 4.11x | 1.45x | 1.66x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.89x | 0.99x | 78.15x | 148.18x | 174.67x | 155.10x | 4/5 |
| chess960_materialize | 0.97x | 1.73x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.48x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.24x | 6.49x | 2.46x | 1.35x | 2.96x | 2.75x | 10/10 |
| corpus_materialize | 1.00x | 1.39x | 1.30x | 1.07x | 1.70x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.28x | 2.97x | 2.24x | 2.85x | 1.56x | 10/10 |
| replay_trusted_native | 1.05x | 0.93x | 1.65x | 1.61x | 2.30x | 2.67x | 9/10 |
| replay_checked_native | 1.02x | 2.22x | 1.24x | 9.47x | 9.09x | 10.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
