| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.25x | 1.54x | 1.48x | 2.34x | 1.88x | 30/30 |
| materialize | 1.03x | 2.02x | 1.35x | 1.15x | 2.29x | 1.51x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.71x | 1.51x | 1.33x | 2.49x | 2.10x | 30/30 |
| perft_full_d3 | 0.98x | 1.24x | 1.27x | 1.08x | 1.72x | 1.55x | 28/30 |
| fen_parse | 1.19x | 1.43x | 1.58x | 1.56x | 2.35x | 1.59x | 30/30 |
| fen_write_native_ep | 0.97x | 3.87x | 7.37x | 1.58x | 30.64x | 2.01x | 30/30 |
| snapshot | 0.66x | 1.52x | 1.16x | 2.35x | 1.16x | 1.00x | 25/30 |
| san_parse_all | 0.98x | 1.86x | N/A | 4.61x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.73x | N/A | 3.57x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.43x | 0.99x | 6.12x | 1.11x | 1.28x | 25/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.17x | 85.43x | 148.49x | 201.65x | 147.46x | 5/5 |
| chess960_materialize | 1.02x | 1.61x | 1.25x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.68x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.21x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 14.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.88x | 4.22x | 1.62x | 1.18x | 2.17x | 2.05x | 9/10 |
| corpus_materialize | 0.91x | 1.71x | 1.33x | 1.08x | 2.68x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.95x | 1.92x | 2.32x | 1.87x | 2.25x | 1.51x | 10/10 |
| replay_trusted_native | 0.98x | 0.73x | 1.28x | 1.62x | 2.62x | 3.40x | 8/10 |
| replay_checked_native | 0.92x | 2.03x | 0.91x | 5.60x | 5.92x | 7.03x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
