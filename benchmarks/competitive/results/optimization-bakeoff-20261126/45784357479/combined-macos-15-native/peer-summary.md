| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.09x | 4.52x | 1.51x | 1.43x | 1.99x | 1.72x | 29/30 |
| materialize | 1.08x | 1.98x | 1.44x | 1.08x | 2.28x | 1.35x | 27/30 |
| perft_bulk_d4 | 1.00x | 3.91x | 1.44x | 1.24x | 1.94x | 1.85x | 29/30 |
| perft_full_d3 | 1.12x | 1.57x | 1.74x | 1.40x | 1.99x | 2.23x | 30/30 |
| fen_parse | 1.10x | 1.28x | 1.46x | 1.33x | 1.59x | 1.47x | 30/30 |
| fen_write_native_ep | 1.04x | 3.89x | 8.54x | 1.64x | 27.52x | 1.98x | 30/30 |
| snapshot | 1.09x | 1.69x | 1.27x | 2.33x | 1.10x | 1.06x | 26/30 |
| san_parse_all | 1.07x | 2.06x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.10x | 11.48x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 0.99x | 1.01x | 4.99x | 1.05x | 1.08x | 21/30 |
| make_unmake_full | 1.20x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.26x | 76.56x | 157.10x | 193.41x | 155.45x | 5/5 |
| chess960_materialize | 0.83x | 1.53x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 3.46x | 1.56x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 39.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.80x | 0.91x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.93x | 24.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 6.36x | 1.88x | 1.21x | 2.25x | 2.80x | 10/10 |
| corpus_materialize | 0.80x | 1.59x | 1.32x | 1.13x | 1.77x | 1.26x | 9/10 |
| corpus_uci_sorted | 0.76x | 1.77x | 2.28x | 1.77x | 2.24x | 1.27x | 10/10 |
| replay_trusted_native | 0.77x | 0.57x | 1.35x | 1.38x | 1.43x | 1.36x | 8/10 |
| replay_checked_native | 0.90x | 2.37x | 1.08x | 7.68x | 7.24x | 8.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
