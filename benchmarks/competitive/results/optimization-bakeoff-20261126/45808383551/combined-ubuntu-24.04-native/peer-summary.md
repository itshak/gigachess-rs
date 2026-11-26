| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.92x | 1.68x | 1.42x | 2.20x | 1.74x | 30/30 |
| materialize | 0.99x | 2.07x | 1.61x | 1.63x | 2.50x | 1.82x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.32x | 1.36x | 1.04x | 2.07x | 1.65x | 29/30 |
| perft_full_d3 | 1.01x | 1.54x | 1.75x | 1.67x | 2.29x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.27x | 1.38x | 1.16x | 1.48x | 1.14x | 30/30 |
| fen_write_native_ep | 0.96x | 4.85x | 9.70x | 1.96x | 27.49x | 2.08x | 30/30 |
| snapshot | 0.92x | 1.73x | 1.08x | 3.05x | 1.09x | 1.16x | 30/30 |
| san_parse_all | 0.99x | 1.90x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.19x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.94x | 1.25x | 3.62x | 1.20x | 1.84x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.15x | 72.31x | 134.36x | 153.70x | 126.33x | 5/5 |
| chess960_materialize | 1.00x | 1.67x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.36x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 21.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.66x | 4.28x | 1.61x | 0.89x | 1.94x | 1.79x | 8/10 |
| corpus_materialize | 0.95x | 1.34x | 1.25x | 1.04x | 1.63x | 1.14x | 9/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.11x | 2.22x | 2.89x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.80x | 1.51x | 1.48x | 2.03x | 2.39x | 9/10 |
| replay_checked_native | 1.00x | 2.10x | 1.11x | 8.77x | 8.50x | 10.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
