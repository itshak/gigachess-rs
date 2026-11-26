| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.74x | 1.45x | 2.46x | 1.78x | 30/30 |
| materialize | 1.01x | 2.10x | 1.60x | 1.64x | 2.58x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.59x | 1.21x | 2.54x | 1.92x | 30/30 |
| perft_full_d3 | 0.97x | 1.48x | 1.66x | 1.63x | 2.19x | 1.80x | 30/30 |
| fen_parse | 1.13x | 1.45x | 1.43x | 1.20x | 1.68x | 1.22x | 30/30 |
| fen_write_native_ep | 0.95x | 4.53x | 10.12x | 1.86x | 29.00x | 2.44x | 30/30 |
| snapshot | 0.77x | 1.55x | 1.00x | 3.48x | 1.00x | 1.15x | 24/30 |
| san_parse_all | 0.99x | 1.90x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.11x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.72x | 1.06x | 3.18x | 1.05x | 1.30x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.08x | 75.91x | 136.22x | 169.21x | 127.68x | 5/5 |
| chess960_materialize | 0.99x | 1.67x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.28x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 39.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.03x | 18.83x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.45x | 2.46x | 1.35x | 2.95x | 2.73x | 10/10 |
| corpus_materialize | 1.00x | 1.42x | 1.34x | 1.09x | 1.74x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.23x | 3.04x | 2.27x | 2.93x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.71x | 1.24x | 1.27x | 1.80x | 2.07x | 8/10 |
| replay_checked_native | 1.01x | 2.03x | 1.06x | 8.58x | 8.21x | 9.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
