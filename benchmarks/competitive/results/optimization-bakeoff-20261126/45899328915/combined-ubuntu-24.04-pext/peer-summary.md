| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.44x | 1.81x | 1.63x | 2.43x | 1.85x | 30/30 |
| materialize | 1.02x | 2.22x | 1.64x | 1.72x | 2.63x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.90x | 1.55x | 1.21x | 2.41x | 1.90x | 29/30 |
| perft_full_d3 | 1.03x | 1.43x | 1.61x | 1.54x | 2.09x | 1.66x | 30/30 |
| fen_parse | 1.00x | 1.39x | 1.59x | 1.06x | 1.47x | 1.25x | 29/30 |
| fen_write_native_ep | 0.92x | 4.57x | 9.52x | 1.94x | 25.89x | 2.41x | 30/30 |
| snapshot | 0.75x | 1.37x | 0.86x | 3.23x | 0.87x | 1.00x | 14/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.75x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.09x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.14x | 1.37x | 3.97x | 1.36x | 1.63x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.82x | 0.91x | 72.00x | 124.57x | 154.08x | 138.94x | 4/5 |
| chess960_materialize | 0.89x | 1.67x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.45x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 39.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.95x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.94x | 21.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.23x | 2.29x | 1.28x | 2.84x | 2.57x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.31x | 1.08x | 1.75x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.31x | 3.02x | 2.22x | 2.94x | 1.57x | 10/10 |
| replay_trusted_native | 0.94x | 0.85x | 1.55x | 1.49x | 2.09x | 2.43x | 9/10 |
| replay_checked_native | 0.89x | 2.00x | 1.04x | 8.26x | 8.06x | 9.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
