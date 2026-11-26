| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.13x | 1.62x | 1.39x | 1.95x | 1.41x | 30/30 |
| materialize | 0.99x | 2.32x | 1.85x | 1.34x | 2.67x | 1.68x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.12x | 1.61x | 1.22x | 1.95x | 1.82x | 30/30 |
| perft_full_d3 | 0.98x | 1.46x | 1.74x | 1.32x | 1.98x | 1.78x | 30/30 |
| fen_parse | 0.99x | 1.53x | 1.47x | 1.16x | 1.79x | 1.15x | 30/30 |
| fen_write_native_ep | 1.02x | 2.67x | 8.39x | 1.44x | 25.47x | 1.97x | 30/30 |
| snapshot | 1.01x | 1.63x | 1.16x | 2.65x | 1.16x | 1.00x | 28/30 |
| san_parse_all | 1.01x | 1.83x | N/A | 3.71x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 10.92x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.32x | 1.54x | 6.54x | 1.35x | 1.39x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.29x | 59.65x | 86.78x | 121.82x | 85.86x | 5/5 |
| chess960_materialize | 0.97x | 1.85x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.29x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 31.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.15x | 24.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.27x | 1.79x | 1.08x | 2.01x | 2.81x | 9/10 |
| corpus_materialize | 0.97x | 1.77x | 1.64x | 1.21x | 2.32x | 1.43x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.05x | 2.85x | 2.32x | 2.80x | 1.58x | 10/10 |
| replay_trusted_native | 1.02x | 0.77x | 1.73x | 1.71x | 2.31x | 2.08x | 9/10 |
| replay_checked_native | 0.97x | 2.17x | 1.25x | 8.45x | 8.40x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
