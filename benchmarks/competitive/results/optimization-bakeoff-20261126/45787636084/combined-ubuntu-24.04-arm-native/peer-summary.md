| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.90x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.01x | 1.83x | 1.45x | 1.26x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.43x | 1.41x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.29x | 1.07x | 1.58x | 1.49x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.33x | 1.28x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.37x | 1.61x | 29.73x | 2.39x | 30/30 |
| snapshot | 0.92x | 1.16x | 1.02x | 1.46x | 1.00x | 0.94x | 19/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.78x | N/A | 2.55x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.22x | 1.43x | 3.95x | 1.18x | 1.17x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.27x | 76.18x | 130.71x | 171.49x | 139.60x | 5/5 |
| chess960_materialize | 1.02x | 1.55x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.66x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.96x | 18.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.92x | 1.52x | 1.21x | 1.79x | 1.97x | 10/10 |
| corpus_materialize | 0.98x | 1.53x | 1.46x | 1.17x | 1.83x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.51x | 1.33x | 1.61x | 1.82x | 1.84x | 8/10 |
| replay_checked_native | 1.00x | 2.04x | 1.00x | 6.72x | 5.96x | 7.72x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
