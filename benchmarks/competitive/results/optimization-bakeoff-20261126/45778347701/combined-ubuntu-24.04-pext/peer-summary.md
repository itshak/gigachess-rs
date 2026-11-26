| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.34x | 1.82x | 1.61x | 2.47x | 1.86x | 30/30 |
| materialize | 0.95x | 1.98x | 1.52x | 1.69x | 2.45x | 1.83x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.70x | 1.46x | 1.15x | 2.30x | 1.81x | 30/30 |
| perft_full_d3 | 1.01x | 1.55x | 1.74x | 1.70x | 2.27x | 1.86x | 30/30 |
| fen_parse | 0.96x | 1.27x | 1.17x | 0.96x | 1.29x | 1.10x | 26/30 |
| fen_write_native_ep | 1.01x | 2.97x | 6.47x | 1.35x | 20.73x | 1.70x | 30/30 |
| snapshot | 1.06x | 1.50x | 0.75x | 5.69x | 0.75x | 1.00x | 15/30 |
| san_parse_all | 0.97x | 1.88x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.04x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 0.69x | 1.24x | 0.83x | 2.26x | 0.81x | 0.97x | 15/30 |
| make_unmake_full | 0.85x | N/A | N/A | 0.97x | N/A | N/A | 1/6 |
| startpos | 0.98x | 1.00x | 78.38x | 135.62x | 171.44x | 154.53x | 5/5 |
| chess960_materialize | 1.00x | 1.58x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.19x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 38.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 20.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.92x | 2.52x | 1.42x | 3.12x | 2.86x | 10/10 |
| corpus_materialize | 1.01x | 1.48x | 1.35x | 1.14x | 1.80x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.30x | 3.05x | 2.27x | 2.98x | 1.60x | 10/10 |
| replay_trusted_native | 0.95x | 0.68x | 1.23x | 1.22x | 1.71x | 2.18x | 8/10 |
| replay_checked_native | 0.95x | 1.88x | 1.01x | 8.21x | 7.78x | 9.38x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
