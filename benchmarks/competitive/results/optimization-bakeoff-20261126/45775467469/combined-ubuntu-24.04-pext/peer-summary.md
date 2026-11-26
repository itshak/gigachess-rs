| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.15x | 1.75x | 1.52x | 2.34x | 1.77x | 30/30 |
| materialize | 1.00x | 2.17x | 1.59x | 1.72x | 2.57x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.90x | 1.55x | 1.23x | 2.41x | 1.91x | 30/30 |
| perft_full_d3 | 1.51x | 1.36x | 1.55x | 1.52x | 1.99x | 1.64x | 30/30 |
| fen_parse | 0.96x | 1.38x | 1.32x | 1.15x | 1.44x | 1.25x | 30/30 |
| fen_write_native_ep | 1.01x | 3.00x | 6.41x | 1.23x | 19.27x | 1.62x | 30/30 |
| snapshot | 0.77x | 1.34x | 1.08x | 2.28x | 1.08x | 1.00x | 27/30 |
| san_parse_all | 0.98x | 1.87x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.06x | N/A | 2.07x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.56x | 0.99x | 3.02x | 0.98x | 1.24x | 24/30 |
| make_unmake_full | 0.99x | N/A | N/A | 0.95x | N/A | N/A | 1/6 |
| startpos | 0.82x | 1.25x | 65.80x | 124.38x | 142.17x | 129.17x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.21x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 38.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.93x | 19.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.69x | 2.45x | 1.39x | 3.08x | 2.75x | 10/10 |
| corpus_materialize | 1.01x | 1.46x | 1.33x | 1.12x | 1.78x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.40x | 1.87x | 1.39x | 1.84x | 0.97x | 8/10 |
| replay_trusted_native | 1.01x | 0.62x | 1.10x | 1.09x | 1.52x | 1.78x | 7/10 |
| replay_checked_native | 0.96x | 1.77x | 0.93x | 7.45x | 7.15x | 8.43x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
