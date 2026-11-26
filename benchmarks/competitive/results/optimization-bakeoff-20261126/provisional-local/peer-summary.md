| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.18x | 3.51x | 1.29x | 1.36x | 1.92x | 1.62x | 30/30 |
| materialize | 1.23x | 1.86x | 1.35x | 1.14x | 1.92x | 1.38x | 30/30 |
| perft_bulk_d4 | 1.57x | 2.94x | 1.17x | 1.07x | 1.60x | 1.40x | 27/30 |
| perft_full_d3 | 1.80x | 0.91x | 1.07x | 0.91x | 1.12x | 1.13x | 16/30 |
| fen_parse | 1.23x | 1.25x | 1.29x | 1.14x | 1.57x | 1.04x | 28/30 |
| fen_write_native_ep | 1.12x | 3.15x | 7.05x | 1.17x | 19.43x | 1.51x | 30/30 |
| snapshot | 1.03x | 1.26x | 0.70x | 1.70x | 0.70x | 0.63x | 12/30 |
| san_parse_all | 1.81x | 1.70x | N/A | 3.19x | N/A | N/A | 12/12 |
| san_render_native | 1.92x | 10.42x | N/A | 1.92x | N/A | N/A | 12/12 |
| apply_children_full | 1.23x | 0.87x | 0.86x | 2.78x | 0.83x | 0.85x | 8/30 |
| make_unmake_full | 1.18x | N/A | N/A | 0.90x | N/A | N/A | 0/6 |
| startpos | 1.01x | 0.95x | 29.48x | 89.97x | 97.21x | 75.36x | 4/5 |
| chess960_materialize | 0.75x | 1.35x | 1.15x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 1.68x | 1.06x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 28.50x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 14.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.23x | 1.67x | 1.33x | 2.24x | 1.99x | 10/10 |
| corpus_materialize | 1.02x | 1.70x | 1.42x | 1.02x | 1.68x | 1.20x | 9/10 |
| corpus_uci_sorted | 1.17x | 1.16x | 1.39x | 1.08x | 1.38x | 0.81x | 8/10 |
| replay_trusted_native | 0.93x | 0.43x | 1.24x | 1.30x | 1.47x | 1.35x | 8/10 |
| replay_checked_native | 0.70x | 1.34x | 0.75x | 3.40x | 3.35x | 4.06x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
