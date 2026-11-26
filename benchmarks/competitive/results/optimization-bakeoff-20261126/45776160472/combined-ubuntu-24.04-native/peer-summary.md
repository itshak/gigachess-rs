| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.54x | 1.43x | 1.34x | 1.80x | 1.59x | 30/30 |
| materialize | 1.00x | 1.97x | 1.52x | 1.31x | 2.16x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.51x | 1.43x | 1.19x | 1.96x | 1.71x | 30/30 |
| perft_full_d3 | 0.97x | 0.89x | 0.98x | 0.86x | 1.22x | 1.11x | 15/30 |
| fen_parse | 0.92x | 1.21x | 1.42x | 1.13x | 1.61x | 1.17x | 30/30 |
| fen_write_native_ep | 1.41x | 4.56x | 9.45x | 1.90x | 28.07x | 2.55x | 30/30 |
| snapshot | 1.12x | 1.79x | 1.14x | 3.70x | 1.87x | 1.14x | 30/30 |
| san_parse_all | 0.98x | 1.88x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.09x | N/A | 2.39x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.38x | 0.84x | 2.94x | 0.84x | 1.03x | 15/30 |
| make_unmake_full | 0.97x | N/A | N/A | 0.99x | N/A | N/A | 2/6 |
| startpos | 1.10x | 1.12x | 101.20x | 185.10x | 246.84x | 180.43x | 5/5 |
| chess960_materialize | 1.01x | 1.49x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.07x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 32.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.07x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.34x | 2.08x | 1.20x | 2.41x | 2.37x | 10/10 |
| corpus_materialize | 1.00x | 1.50x | 1.43x | 1.12x | 1.76x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.40x | 1.76x | 1.42x | 1.70x | 0.98x | 8/10 |
| replay_trusted_native | 0.98x | 0.78x | 1.05x | 1.22x | 1.72x | 1.88x | 7/10 |
| replay_checked_native | 1.02x | 1.77x | 0.90x | 6.67x | 6.11x | 7.57x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
