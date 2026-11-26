| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.83x | 1.46x | 1.58x | 1.90x | 1.89x | 30/30 |
| materialize | 1.00x | 1.81x | 1.41x | 1.25x | 2.00x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.71x | 1.53x | 1.37x | 1.94x | 2.01x | 30/30 |
| perft_full_d3 | 0.99x | 1.16x | 1.27x | 1.07x | 1.55x | 1.49x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.32x | 1.26x | 1.78x | 1.40x | 30/30 |
| fen_write_native_ep | 1.51x | 4.50x | 9.30x | 1.59x | 29.92x | 2.39x | 30/30 |
| snapshot | 0.98x | 1.20x | 1.01x | 1.60x | 1.01x | 1.02x | 25/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.74x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.12x | 1.31x | 3.63x | 1.08x | 1.09x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.17x | 71.14x | 121.08x | 164.46x | 131.68x | 5/5 |
| chess960_materialize | 1.01x | 1.51x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.61x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.43x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.90x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.70x | 1.83x | 1.47x | 2.17x | 2.39x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.50x | 1.21x | 1.88x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.90x | 2.08x | 2.80x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.52x | 1.35x | 1.61x | 1.82x | 1.85x | 8/10 |
| replay_checked_native | 0.99x | 2.00x | 0.99x | 6.67x | 5.94x | 7.68x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
