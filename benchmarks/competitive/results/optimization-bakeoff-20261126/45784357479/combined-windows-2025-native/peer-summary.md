| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.31x | 1.57x | 1.51x | 2.36x | 1.90x | 30/30 |
| materialize | 1.01x | 1.94x | 1.28x | 1.08x | 2.18x | 1.39x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.70x | 1.50x | 1.33x | 2.30x | 2.07x | 30/30 |
| perft_full_d3 | 0.98x | 1.22x | 1.27x | 1.04x | 1.71x | 1.50x | 27/30 |
| fen_parse | 1.20x | 1.49x | 1.58x | 1.49x | 2.52x | 1.61x | 30/30 |
| fen_write_native_ep | 1.01x | 3.95x | 8.07x | 1.63x | 31.00x | 1.98x | 30/30 |
| snapshot | 0.83x | 1.67x | 1.24x | 2.70x | 1.20x | 1.10x | 30/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.61x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.70x | N/A | 3.44x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.49x | 1.04x | 6.06x | 1.14x | 1.31x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 2.44x | 1.08x | 81.05x | 133.53x | 208.78x | 147.33x | 5/5 |
| chess960_materialize | 1.01x | 1.49x | 1.21x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.56x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.86x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 16.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.39x | 1.79x | 1.32x | 2.36x | 2.15x | 10/10 |
| corpus_materialize | 0.99x | 1.85x | 1.35x | 1.15x | 2.75x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.87x | 2.37x | 1.94x | 2.29x | 1.49x | 10/10 |
| replay_trusted_native | 0.99x | 0.75x | 1.28x | 1.59x | 2.71x | 3.28x | 8/10 |
| replay_checked_native | 0.99x | 2.13x | 0.92x | 5.31x | 6.03x | 7.65x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
