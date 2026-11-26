| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.90x | 4.05x | 1.70x | 1.47x | 2.44x | 1.73x | 30/30 |
| materialize | 0.95x | 2.15x | 1.59x | 1.69x | 2.62x | 2.00x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.57x | 1.42x | 1.11x | 2.22x | 1.73x | 29/30 |
| perft_full_d3 | 0.92x | 1.22x | 1.47x | 1.38x | 1.81x | 1.47x | 30/30 |
| fen_parse | 1.03x | 1.52x | 1.39x | 1.18x | 1.53x | 1.15x | 30/30 |
| fen_write_native_ep | 0.90x | 4.40x | 9.46x | 1.74x | 25.67x | 2.27x | 30/30 |
| snapshot | 0.75x | 1.84x | 1.15x | 3.24x | 1.15x | 1.23x | 30/30 |
| san_parse_all | 0.97x | 1.95x | N/A | 4.07x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.31x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.73x | 1.59x | 1.03x | 2.93x | 1.03x | 1.70x | 26/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.25x | 1.59x | 103.10x | 179.24x | 220.92x | 176.48x | 5/5 |
| chess960_materialize | 0.97x | 1.65x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.36x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 38.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 21.96x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 4.91x | 1.80x | 0.99x | 2.24x | 2.06x | 9/10 |
| corpus_materialize | 0.99x | 1.40x | 1.29x | 1.08x | 1.70x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.05x | 2.40x | 3.09x | 2.28x | 2.99x | 1.61x | 10/10 |
| replay_trusted_native | 0.99x | 0.87x | 1.62x | 1.61x | 2.25x | 2.53x | 9/10 |
| replay_checked_native | 0.89x | 1.95x | 1.07x | 8.27x | 7.94x | 9.22x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
