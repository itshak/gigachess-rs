| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.09x | 1.77x | 1.81x | 2.39x | 2.10x | 30/30 |
| materialize | 1.00x | 1.99x | 1.49x | 1.11x | 2.30x | 1.38x | 28/30 |
| perft_bulk_d4 | 1.02x | 4.39x | 1.61x | 1.30x | 2.02x | 1.95x | 30/30 |
| perft_full_d3 | 1.12x | 1.73x | 1.92x | 1.48x | 2.17x | 2.25x | 30/30 |
| fen_parse | 0.96x | 1.28x | 1.38x | 1.35x | 1.70x | 1.39x | 30/30 |
| fen_write_native_ep | 0.97x | 4.27x | 9.02x | 1.74x | 27.52x | 2.25x | 30/30 |
| snapshot | 0.98x | 1.71x | 1.12x | 2.41x | 1.17x | 0.98x | 24/30 |
| san_parse_all | 0.93x | 1.67x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.45x | N/A | 2.91x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.18x | 1.26x | 5.97x | 1.19x | 1.35x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.20x | 66.70x | 132.85x | 159.57x | 128.45x | 5/5 |
| chess960_materialize | 0.99x | 1.79x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.38x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 38.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.91x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 19.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 6.70x | 2.15x | 1.33x | 2.56x | 2.80x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.42x | 1.18x | 1.98x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.00x | 3.12x | 2.15x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.66x | 1.73x | 1.72x | 1.84x | 1.83x | 8/10 |
| replay_checked_native | 1.00x | 2.67x | 1.28x | 9.75x | 8.80x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
