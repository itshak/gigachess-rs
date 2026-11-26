| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.43x | 1.51x | 1.46x | 2.17x | 1.79x | 30/30 |
| materialize | 1.06x | 2.08x | 1.65x | 1.17x | 2.30x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.86x | 1.47x | 1.17x | 2.22x | 2.04x | 30/30 |
| perft_full_d3 | 1.11x | 1.33x | 1.57x | 1.18x | 2.01x | 1.93x | 30/30 |
| fen_parse | 0.99x | 1.39x | 1.60x | 1.24x | 2.14x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.01x | 8.63x | 1.48x | 25.16x | 1.90x | 30/30 |
| snapshot | 1.08x | 2.50x | 1.41x | 7.92x | 1.42x | 1.44x | 30/30 |
| san_parse_all | 0.99x | 1.98x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.09x | 11.03x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.50x | 1.78x | 1.30x | 3.84x | 1.27x | 1.81x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.25x | 84.48x | 140.03x | 212.48x | 149.28x | 5/5 |
| chess960_materialize | 1.10x | 1.62x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.67x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 36.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.55x | 14.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 5.56x | 1.60x | 1.09x | 2.12x | 1.98x | 9/10 |
| corpus_materialize | 0.84x | 1.59x | 1.61x | 1.07x | 2.06x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.36x | 3.07x | 2.43x | 3.03x | 1.65x | 10/10 |
| replay_trusted_native | 1.01x | 0.66x | 1.29x | 1.41x | 2.20x | 2.61x | 8/10 |
| replay_checked_native | 1.02x | 2.19x | 0.96x | 7.48x | 7.40x | 8.98x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
