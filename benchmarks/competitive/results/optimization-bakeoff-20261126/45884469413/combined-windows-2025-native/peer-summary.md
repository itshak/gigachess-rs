| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.90x | 1.73x | 1.66x | 2.42x | 2.14x | 30/30 |
| materialize | 1.01x | 2.05x | 1.39x | 1.37x | 2.43x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.11x | 1.72x | 1.39x | 2.36x | 2.09x | 30/30 |
| perft_full_d3 | 1.04x | 1.41x | 1.50x | 1.37x | 1.96x | 1.91x | 30/30 |
| fen_parse | 1.08x | 1.28x | 1.49x | 1.41x | 2.25x | 1.49x | 30/30 |
| fen_write_native_ep | 1.03x | 4.10x | 8.02x | 1.68x | 32.39x | 2.00x | 30/30 |
| snapshot | 1.01x | 1.82x | 1.02x | 3.98x | 1.00x | 1.01x | 26/30 |
| san_parse_all | 1.06x | 2.14x | N/A | 5.62x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.92x | N/A | 3.83x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.33x | 1.40x | 9.50x | 1.49x | 1.67x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.22x | 163.75x | 240.00x | 173.34x | 5/5 |
| chess960_materialize | 1.23x | 1.65x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.91x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 27.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.11x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 5.62x | 1.86x | 1.41x | 2.57x | 2.47x | 10/10 |
| corpus_materialize | 1.13x | 1.61x | 1.29x | 1.20x | 2.03x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.09x | 1.82x | 2.24x | 1.87x | 2.20x | 1.45x | 10/10 |
| replay_trusted_native | 1.05x | 0.85x | 1.63x | 2.11x | 2.63x | 3.04x | 9/10 |
| replay_checked_native | 1.00x | 2.49x | 1.10x | 8.73x | 8.24x | 9.74x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
