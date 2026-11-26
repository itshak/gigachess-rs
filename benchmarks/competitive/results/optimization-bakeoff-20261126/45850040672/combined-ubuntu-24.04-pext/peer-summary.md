| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 5.01x | 1.73x | 1.76x | 2.75x | 2.01x | 30/30 |
| materialize | 1.00x | 2.22x | 1.77x | 1.21x | 2.37x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.35x | 1.68x | 1.41x | 2.72x | 2.19x | 30/30 |
| perft_full_d3 | 1.00x | 1.51x | 1.58x | 1.20x | 2.27x | 1.79x | 30/30 |
| fen_parse | 1.02x | 1.44x | 1.58x | 1.20x | 2.39x | 1.26x | 30/30 |
| fen_write_native_ep | 0.99x | 4.05x | 9.88x | 1.86x | 27.86x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.91x | 1.25x | 3.79x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.41x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.48x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.98x | 1.31x | 4.74x | 1.52x | 2.03x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 99.24x | 136.52x | 234.75x | 139.96x | 5/5 |
| chess960_materialize | 1.00x | 1.92x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.15x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 20.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.00x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.63x | 1.78x | 1.38x | 2.50x | 2.04x | 9/10 |
| corpus_materialize | 1.00x | 1.94x | 1.78x | 1.27x | 2.91x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.19x | 2.37x | 3.09x | 1.67x | 10/10 |
| replay_trusted_native | 1.09x | 1.18x | 2.02x | 2.28x | 4.08x | 4.63x | 10/10 |
| replay_checked_native | 1.08x | 2.91x | 1.09x | 6.34x | 7.36x | 8.36x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
