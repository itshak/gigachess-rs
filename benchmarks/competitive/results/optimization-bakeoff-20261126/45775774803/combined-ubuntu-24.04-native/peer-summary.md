| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.85x | 1.65x | 1.42x | 2.17x | 1.72x | 30/30 |
| materialize | 1.01x | 2.03x | 1.52x | 1.61x | 2.35x | 1.74x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.79x | 1.55x | 1.18x | 2.30x | 1.86x | 30/30 |
| perft_full_d3 | 0.97x | 0.87x | 1.01x | 0.94x | 1.27x | 1.07x | 16/30 |
| fen_parse | 0.89x | 1.12x | 1.22x | 0.97x | 1.36x | 1.08x | 25/30 |
| fen_write_native_ep | 1.00x | 3.32x | 6.84x | 1.41x | 20.10x | 1.82x | 30/30 |
| snapshot | 1.05x | 1.36x | 1.11x | 3.10x | 1.11x | 1.11x | 30/30 |
| san_parse_all | 1.03x | 1.78x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 8.94x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.21x | 1.57x | 0.99x | 2.99x | 1.01x | 1.22x | 25/30 |
| make_unmake_full | 1.00x | N/A | N/A | 0.93x | N/A | N/A | 1/6 |
| startpos | 0.89x | 1.08x | 74.20x | 120.50x | 160.75x | 128.27x | 5/5 |
| chess960_materialize | 1.00x | 1.57x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.17x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 38.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.23x | 29.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.37x | 2.50x | 1.28x | 3.01x | 2.71x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.32x | 1.09x | 1.71x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.84x | 1.38x | 1.78x | 0.98x | 8/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.09x | 1.07x | 1.52x | 1.77x | 7/10 |
| replay_checked_native | 1.09x | 1.90x | 1.05x | 8.07x | 7.86x | 9.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
