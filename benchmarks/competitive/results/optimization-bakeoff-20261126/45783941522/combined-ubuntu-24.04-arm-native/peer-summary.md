| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.84x | 1.46x | 1.57x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.83x | 1.44x | 1.25x | 2.02x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.72x | 1.52x | 1.37x | 1.94x | 1.99x | 30/30 |
| perft_full_d3 | 1.00x | 1.18x | 1.28x | 1.07x | 1.57x | 1.48x | 29/30 |
| fen_parse | 1.00x | 1.25x | 1.33x | 1.29x | 1.78x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.37x | 1.60x | 29.77x | 2.38x | 30/30 |
| snapshot | 1.02x | 1.28x | 0.93x | 1.68x | 0.91x | 1.01x | 18/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.76x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.13x | 1.30x | 3.66x | 1.09x | 1.10x | 29/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.18x | 76.00x | 130.00x | 170.53x | 136.63x | 5/5 |
| chess960_materialize | 1.00x | 1.51x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.60x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.69x | 1.81x | 1.46x | 2.12x | 2.35x | 10/10 |
| corpus_materialize | 1.00x | 1.56x | 1.50x | 1.20x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.79x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.51x | 1.33x | 1.61x | 1.80x | 1.84x | 8/10 |
| replay_checked_native | 0.98x | 1.99x | 0.98x | 6.66x | 5.85x | 7.61x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
