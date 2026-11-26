| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.52x | 1.50x | 1.43x | 2.12x | 1.92x | 30/30 |
| materialize | 0.98x | 2.05x | 1.35x | 1.15x | 2.28x | 1.43x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.99x | 1.51x | 1.23x | 2.29x | 2.10x | 30/30 |
| perft_full_d3 | 1.12x | 1.10x | 1.22x | 0.98x | 1.63x | 1.72x | 24/30 |
| fen_parse | 1.21x | 1.52x | 1.68x | 1.64x | 2.31x | 1.73x | 30/30 |
| fen_write_native_ep | 0.98x | 3.67x | 7.48x | 1.53x | 32.05x | 2.05x | 30/30 |
| snapshot | 1.08x | 2.05x | 1.35x | 3.72x | 1.27x | 1.23x | 30/30 |
| san_parse_all | 0.94x | 1.86x | N/A | 4.84x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.41x | N/A | 3.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.34x | 1.43x | 1.09x | 6.53x | 1.07x | 1.50x | 28/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.26x | 90.48x | 177.26x | 239.73x | 184.06x | 5/5 |
| chess960_materialize | 1.01x | 1.44x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.57x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 36.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 24.95x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.06x | 1.83x | 1.26x | 2.37x | 2.30x | 10/10 |
| corpus_materialize | 0.97x | 1.68x | 1.32x | 1.14x | 2.24x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.62x | 2.03x | 1.75x | 1.94x | 1.38x | 10/10 |
| replay_trusted_native | 0.98x | 0.65x | 1.20x | 1.43x | 2.15x | 2.62x | 8/10 |
| replay_checked_native | 1.02x | 2.00x | 0.97x | 7.08x | 7.00x | 8.91x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
