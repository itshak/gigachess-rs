| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.13x | 1.50x | 1.45x | 2.15x | 1.82x | 30/30 |
| materialize | 0.99x | 1.91x | 1.29x | 1.09x | 2.18x | 1.39x | 29/30 |
| perft_bulk_d4 | 1.01x | 3.81x | 1.53x | 1.35x | 2.43x | 2.14x | 30/30 |
| perft_full_d3 | 0.97x | 0.86x | 0.76x | 0.61x | 1.03x | 0.95x | 7/30 |
| fen_parse | 0.98x | 1.43x | 1.60x | 1.54x | 2.30x | 1.58x | 30/30 |
| fen_write_native_ep | 0.99x | 3.00x | 6.26x | 1.21x | 23.62x | 1.51x | 30/30 |
| snapshot | 0.66x | 1.61x | 1.16x | 2.38x | 1.16x | 0.99x | 25/30 |
| san_parse_all | 1.00x | 1.72x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.32x | N/A | 3.47x | N/A | N/A | 12/12 |
| apply_children_full | 1.12x | 1.16x | 0.83x | 4.90x | 0.91x | 1.01x | 15/30 |
| make_unmake_full | 0.98x | N/A | N/A | 0.90x | N/A | N/A | 1/6 |
| startpos | 2.45x | 1.16x | 77.20x | 141.10x | 206.27x | 154.82x | 5/5 |
| chess960_materialize | 1.02x | 1.44x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.46x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 17.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.29x | 1.72x | 1.25x | 2.25x | 2.05x | 10/10 |
| corpus_materialize | 0.94x | 1.76x | 1.28x | 1.09x | 2.60x | 1.41x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.21x | 1.53x | 1.25x | 1.45x | 0.97x | 8/10 |
| replay_trusted_native | 1.00x | 0.51x | 0.87x | 1.08x | 1.77x | 2.24x | 5/10 |
| replay_checked_native | 0.78x | 1.58x | 0.70x | 3.93x | 4.70x | 5.52x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
