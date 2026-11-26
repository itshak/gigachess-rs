| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.37x | 1.67x | 1.79x | 2.16x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.59x | 1.35x | 2.17x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.80x | 1.56x | 1.42x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.00x | 0.92x | 1.00x | 0.84x | 1.23x | 1.15x | 17/30 |
| fen_parse | 1.00x | 1.21x | 1.30x | 1.24x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.01x | 4.47x | 9.22x | 1.59x | 30.16x | 2.40x | 30/30 |
| snapshot | 1.02x | 1.16x | 1.01x | 1.49x | 1.00x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.61x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.46x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.65x | 130.17x | 175.50x | 142.38x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.94x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.32x | 1.67x | 1.37x | 1.96x | 2.29x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.53x | 1.25x | 1.92x | 1.43x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.86x | 1.47x | 10/10 |
| replay_trusted_native | 1.00x | 0.61x | 1.61x | 1.97x | 2.20x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.15x | 1.05x | 7.21x | 6.32x | 8.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
