| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.33x | 1.82x | 1.92x | 2.63x | 2.12x | 30/30 |
| materialize | 1.00x | 2.08x | 1.75x | 1.17x | 2.36x | 1.36x | 29/30 |
| perft_bulk_d4 | 0.98x | 4.13x | 1.57x | 1.22x | 1.95x | 1.80x | 29/30 |
| perft_full_d3 | 1.14x | 1.79x | 2.09x | 1.63x | 2.44x | 2.31x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.45x | 1.29x | 1.72x | 1.60x | 30/30 |
| fen_write_native_ep | 0.93x | 4.09x | 8.97x | 1.58x | 25.34x | 2.07x | 30/30 |
| snapshot | 0.93x | 1.70x | 1.03x | 2.22x | 1.01x | 0.93x | 24/30 |
| san_parse_all | 0.98x | 1.78x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.65x | N/A | 2.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.12x | 1.22x | 5.61x | 1.16x | 1.31x | 28/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.72x | 85.93x | 157.16x | 186.03x | 181.60x | 5/5 |
| chess960_materialize | 0.83x | 1.61x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 3.05x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 40.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.21x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 21.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.84x | 5.61x | 2.00x | 1.13x | 2.07x | 2.38x | 9/10 |
| corpus_materialize | 0.94x | 1.39x | 1.28x | 1.01x | 1.75x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.83x | 1.88x | 2.41x | 1.75x | 2.24x | 1.41x | 10/10 |
| replay_trusted_native | 0.81x | 0.59x | 1.63x | 1.53x | 1.84x | 1.73x | 8/10 |
| replay_checked_native | 0.98x | 2.92x | 1.29x | 9.80x | 9.42x | 10.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
