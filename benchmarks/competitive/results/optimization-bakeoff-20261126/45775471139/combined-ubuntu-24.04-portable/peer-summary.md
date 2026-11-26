| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 2.68x | 1.39x | 1.28x | 1.76x | 1.22x | 28/30 |
| materialize | 0.90x | 1.99x | 1.61x | 1.21x | 2.10x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.76x | 1.48x | 1.17x | 1.85x | 1.59x | 30/30 |
| perft_full_d3 | 0.95x | 0.76x | 0.92x | 0.71x | 1.07x | 1.00x | 8/30 |
| fen_parse | 0.93x | 1.42x | 1.43x | 1.16x | 2.18x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 1.87x | 6.27x | 1.21x | 17.74x | 1.44x | 30/30 |
| snapshot | 1.13x | 1.91x | 1.37x | 3.23x | 1.29x | 1.02x | 29/30 |
| san_parse_all | 0.98x | 2.02x | N/A | 3.48x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.20x | N/A | 2.35x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.48x | 0.95x | 3.54x | 0.88x | 0.87x | 14/30 |
| make_unmake_full | 0.94x | N/A | N/A | 0.95x | N/A | N/A | 1/6 |
| startpos | 1.08x | 1.38x | 78.54x | 117.38x | 194.15x | 121.63x | 5/5 |
| chess960_materialize | 0.98x | 1.51x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.03x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 22.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 2.97x | 1.79x | 1.16x | 1.82x | 1.80x | 10/10 |
| corpus_materialize | 0.99x | 1.96x | 1.87x | 1.33x | 2.90x | 1.70x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.24x | 1.78x | 1.34x | 1.67x | 0.96x | 8/10 |
| replay_trusted_native | 0.96x | 0.53x | 1.10x | 1.30x | 1.78x | 1.73x | 8/10 |
| replay_checked_native | 0.91x | 2.06x | 0.95x | 4.84x | 5.67x | 6.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
