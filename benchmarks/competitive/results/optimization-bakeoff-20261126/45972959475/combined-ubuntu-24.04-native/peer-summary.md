| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.66x | 1.69x | 1.64x | 2.57x | 2.03x | 30/30 |
| materialize | 1.02x | 1.93x | 1.65x | 1.29x | 2.25x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.02x | 1.62x | 1.37x | 2.44x | 2.02x | 30/30 |
| perft_full_d3 | 1.02x | 1.38x | 1.57x | 1.36x | 2.00x | 1.95x | 30/30 |
| fen_parse | 0.99x | 1.20x | 1.23x | 1.07x | 1.90x | 1.16x | 29/30 |
| fen_write_native_ep | 0.99x | 4.06x | 9.67x | 1.71x | 28.09x | 2.03x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.25x | 3.46x | 1.25x | 1.27x | 30/30 |
| san_parse_all | 1.00x | 2.10x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 10.50x | N/A | 2.23x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.31x | 1.32x | 4.61x | 1.48x | 1.62x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.72x | 139.63x | 212.63x | 149.61x | 5/5 |
| chess960_materialize | 1.00x | 1.59x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.85x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.50x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.46x | 1.85x | 1.37x | 2.67x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.52x | 1.14x | 2.06x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.48x | 3.20x | 2.43x | 3.08x | 1.72x | 10/10 |
| replay_trusted_native | 0.95x | 0.85x | 1.55x | 1.94x | 2.48x | 2.81x | 9/10 |
| replay_checked_native | 0.98x | 2.36x | 1.06x | 8.02x | 8.06x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
