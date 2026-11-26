| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.92x | 1.66x | 1.72x | 2.46x | 2.07x | 30/30 |
| materialize | 0.99x | 2.05x | 1.57x | 1.08x | 2.24x | 1.43x | 27/30 |
| perft_bulk_d4 | 0.98x | 4.05x | 1.48x | 1.23x | 1.92x | 1.85x | 30/30 |
| perft_full_d3 | 0.98x | 1.54x | 1.74x | 1.30x | 1.95x | 1.99x | 30/30 |
| fen_parse | 1.04x | 1.30x | 1.43x | 1.35x | 1.68x | 1.45x | 30/30 |
| fen_write_native_ep | 0.98x | 4.14x | 8.93x | 1.76x | 27.62x | 2.24x | 30/30 |
| snapshot | 0.97x | 1.66x | 1.11x | 2.41x | 1.11x | 0.97x | 25/30 |
| san_parse_all | 1.02x | 1.74x | N/A | 3.67x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.63x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.07x | 1.12x | 5.38x | 1.09x | 1.24x | 28/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.20x | 86.20x | 135.77x | 161.22x | 131.06x | 5/5 |
| chess960_materialize | 0.97x | 1.77x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.23x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 40.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 21.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 5.83x | 1.91x | 1.21x | 2.30x | 2.43x | 10/10 |
| corpus_materialize | 0.93x | 1.52x | 1.44x | 1.16x | 1.91x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.82x | 2.58x | 1.81x | 2.49x | 1.37x | 10/10 |
| replay_trusted_native | 1.02x | 0.68x | 1.76x | 1.76x | 1.85x | 1.80x | 8/10 |
| replay_checked_native | 0.96x | 2.48x | 1.14x | 8.81x | 7.74x | 9.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
