| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.84x | 1.67x | 1.41x | 2.15x | 1.72x | 30/30 |
| materialize | 0.96x | 2.02x | 1.54x | 1.61x | 2.42x | 1.79x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.59x | 1.21x | 2.41x | 1.92x | 30/30 |
| perft_full_d3 | 1.03x | 1.52x | 1.73x | 1.70x | 2.39x | 1.87x | 30/30 |
| fen_parse | 1.05x | 1.33x | 1.31x | 1.10x | 1.52x | 1.19x | 30/30 |
| fen_write_native_ep | 0.99x | 5.06x | 10.69x | 1.92x | 31.14x | 2.51x | 30/30 |
| snapshot | 1.29x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 21/30 |
| san_parse_all | 0.93x | 1.75x | N/A | 4.01x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.24x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.71x | 1.05x | 3.08x | 1.02x | 1.24x | 26/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.00x | 78.66x | 150.72x | 183.84x | 155.90x | 4/5 |
| chess960_materialize | 1.05x | 1.66x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.56x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 38.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.12x | 20.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.46x | 2.42x | 1.35x | 2.93x | 2.74x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.10x | 1.74x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.30x | 3.14x | 2.29x | 3.05x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.72x | 1.24x | 1.27x | 1.81x | 2.11x | 8/10 |
| replay_checked_native | 1.01x | 2.06x | 1.04x | 8.64x | 8.20x | 9.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
