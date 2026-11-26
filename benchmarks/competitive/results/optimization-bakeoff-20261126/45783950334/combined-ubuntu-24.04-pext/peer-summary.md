| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.78x | 1.55x | 2.43x | 1.83x | 30/30 |
| materialize | 0.99x | 2.20x | 1.68x | 1.71x | 2.69x | 1.86x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.04x | 1.61x | 1.26x | 2.50x | 1.96x | 30/30 |
| perft_full_d3 | 0.99x | 1.50x | 1.75x | 1.69x | 2.21x | 1.80x | 30/30 |
| fen_parse | 1.10x | 1.53x | 1.62x | 1.21x | 1.54x | 1.18x | 30/30 |
| fen_write_native_ep | 0.96x | 4.35x | 9.62x | 1.86x | 26.96x | 2.45x | 30/30 |
| snapshot | 1.00x | 1.58x | 1.24x | 2.83x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 1.00x | 1.94x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.24x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.72x | 1.12x | 3.19x | 1.07x | 1.36x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 80.59x | 142.39x | 176.63x | 142.44x | 5/5 |
| chess960_materialize | 0.93x | 1.66x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.39x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 37.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 22.25x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.80x | 2.53x | 1.40x | 3.15x | 2.86x | 10/10 |
| corpus_materialize | 0.99x | 1.45x | 1.33x | 1.10x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.27x | 2.96x | 2.24x | 2.86x | 1.58x | 10/10 |
| replay_trusted_native | 0.93x | 0.65x | 1.21x | 1.20x | 1.65x | 2.10x | 7/10 |
| replay_checked_native | 0.99x | 1.95x | 1.06x | 8.26x | 8.02x | 9.42x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
