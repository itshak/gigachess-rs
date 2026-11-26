| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.21x | 4.54x | 1.55x | 1.68x | 2.11x | 2.00x | 30/30 |
| materialize | 1.00x | 2.02x | 1.41x | 0.99x | 2.13x | 1.21x | 27/30 |
| perft_bulk_d4 | 1.04x | 4.48x | 1.72x | 1.30x | 2.26x | 2.00x | 30/30 |
| perft_full_d3 | 0.93x | 1.58x | 1.77x | 1.31x | 2.01x | 1.92x | 30/30 |
| fen_parse | 0.97x | 1.26x | 1.35x | 1.34x | 1.82x | 1.49x | 29/30 |
| fen_write_native_ep | 0.94x | 3.05x | 5.75x | 1.13x | 19.18x | 1.55x | 30/30 |
| snapshot | 1.05x | 1.67x | 1.15x | 2.37x | 1.17x | 0.99x | 22/30 |
| san_parse_all | 0.90x | 1.95x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.03x | N/A | 2.82x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.02x | 1.13x | 5.06x | 1.06x | 1.16x | 24/30 |
| make_unmake_full | 1.07x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 2.12x | 1.95x | 83.11x | 166.03x | 207.11x | 175.79x | 5/5 |
| chess960_materialize | 1.05x | 1.54x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.77x | 1.09x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.16x | 46.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 6.70x | 2.14x | 1.31x | 2.38x | 2.46x | 10/10 |
| corpus_materialize | 1.02x | 1.40x | 1.30x | 1.14x | 1.82x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.64x | 2.36x | 1.75x | 2.16x | 1.22x | 10/10 |
| replay_trusted_native | 1.01x | 0.55x | 1.57x | 1.54x | 1.57x | 1.65x | 8/10 |
| replay_checked_native | 1.28x | 2.36x | 1.05x | 8.12x | 7.19x | 8.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
