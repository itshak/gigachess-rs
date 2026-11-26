| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 3.89x | 1.42x | 1.35x | 2.05x | 1.68x | 30/30 |
| materialize | 0.99x | 1.78x | 1.51x | 1.12x | 2.03x | 1.38x | 29/30 |
| perft_bulk_d4 | 0.97x | 3.57x | 1.45x | 1.22x | 2.12x | 1.78x | 30/30 |
| perft_full_d3 | 0.90x | 0.73x | 0.85x | 0.72x | 1.05x | 1.03x | 10/30 |
| fen_parse | 0.91x | 1.25x | 1.19x | 1.10x | 1.84x | 1.11x | 30/30 |
| fen_write_native_ep | 1.00x | 2.26x | 5.10x | 0.94x | 15.32x | 1.10x | 25/30 |
| snapshot | 0.73x | 1.55x | 1.15x | 2.52x | 1.15x | 1.00x | 28/30 |
| san_parse_all | 0.93x | 1.82x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 10.30x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.62x | 0.98x | 2.95x | 1.23x | 1.05x | 26/30 |
| make_unmake_full | 0.95x | N/A | N/A | 0.97x | N/A | N/A | 2/6 |
| startpos | 0.93x | 1.20x | 72.76x | 137.28x | 209.32x | 140.71x | 5/5 |
| chess960_materialize | 0.95x | 1.33x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.29x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 30.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 22.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.08x | 1.73x | 1.29x | 2.41x | 2.27x | 10/10 |
| corpus_materialize | 0.98x | 1.58x | 1.55x | 1.14x | 2.08x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.40x | 1.83x | 1.38x | 1.76x | 0.99x | 8/10 |
| replay_trusted_native | 0.92x | 0.58x | 1.03x | 1.23x | 1.66x | 1.88x | 8/10 |
| replay_checked_native | 0.97x | 2.02x | 0.89x | 6.77x | 6.66x | 8.20x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
