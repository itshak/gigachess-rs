| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 5.15x | 1.73x | 1.77x | 2.77x | 2.08x | 30/30 |
| materialize | 0.99x | 1.98x | 1.69x | 1.27x | 2.35x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.26x | 1.62x | 1.52x | 2.54x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.52x | 1.33x | 1.97x | 1.89x | 30/30 |
| fen_parse | 1.00x | 1.45x | 1.56x | 1.24x | 2.25x | 1.36x | 30/30 |
| fen_write_native_ep | 0.95x | 4.14x | 9.55x | 1.65x | 28.48x | 2.10x | 30/30 |
| snapshot | 0.72x | 1.43x | 0.90x | 2.92x | 0.90x | 0.90x | 12/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.58x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.47x | 1.33x | 4.62x | 1.47x | 1.68x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.89x | 148.56x | 233.09x | 158.68x | 5/5 |
| chess960_materialize | 1.00x | 1.61x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.88x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 29.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 20.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.07x | 1.93x | 1.50x | 2.98x | 2.53x | 10/10 |
| corpus_materialize | 0.98x | 1.61x | 1.53x | 1.18x | 2.03x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.48x | 3.25x | 2.44x | 3.16x | 1.74x | 10/10 |
| replay_trusted_native | 0.97x | 0.87x | 1.57x | 1.90x | 2.55x | 3.05x | 9/10 |
| replay_checked_native | 0.98x | 2.56x | 1.11x | 8.45x | 8.48x | 9.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
