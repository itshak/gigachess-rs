| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.19x | 1.53x | 1.48x | 2.28x | 1.78x | 30/30 |
| materialize | 0.97x | 1.99x | 1.62x | 1.15x | 2.05x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.85x | 1.51x | 1.25x | 2.38x | 2.05x | 30/30 |
| perft_full_d3 | 0.96x | 1.21x | 1.26x | 0.96x | 1.78x | 1.52x | 26/30 |
| fen_parse | 1.06x | 1.48x | 1.63x | 1.29x | 1.99x | 1.29x | 30/30 |
| fen_write_native_ep | 1.00x | 4.19x | 9.57x | 1.92x | 27.89x | 2.02x | 30/30 |
| snapshot | 0.65x | 1.54x | 1.15x | 2.41x | 1.15x | 0.97x | 24/30 |
| san_parse_all | 1.00x | 2.33x | N/A | 3.76x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.67x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.52x | 1.02x | 3.68x | 1.16x | 1.30x | 27/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.83x | 1.17x | 81.01x | 123.52x | 175.03x | 121.07x | 5/5 |
| chess960_materialize | 1.05x | 1.67x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.72x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 18.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.57x | 1.81x | 1.32x | 2.51x | 2.10x | 10/10 |
| corpus_materialize | 0.98x | 1.92x | 1.71x | 1.21x | 2.81x | 1.56x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.35x | 3.18x | 2.35x | 2.96x | 1.69x | 10/10 |
| replay_trusted_native | 0.88x | 0.65x | 1.13x | 1.30x | 2.25x | 2.78x | 8/10 |
| replay_checked_native | 0.97x | 2.48x | 0.91x | 5.21x | 6.08x | 7.16x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
