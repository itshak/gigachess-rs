| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.19x | 1.58x | 1.72x | 2.07x | 2.05x | 30/30 |
| materialize | 0.99x | 1.93x | 1.53x | 1.32x | 2.14x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.68x | 1.51x | 1.37x | 1.92x | 1.97x | 30/30 |
| perft_full_d3 | 0.88x | 1.30x | 1.41x | 1.19x | 1.72x | 1.64x | 30/30 |
| fen_parse | 0.98x | 1.21x | 1.31x | 1.23x | 1.72x | 1.36x | 30/30 |
| fen_write_native_ep | 0.94x | 4.35x | 8.80x | 1.50x | 28.07x | 2.25x | 30/30 |
| snapshot | 1.07x | 1.17x | 1.03x | 1.54x | 0.98x | 1.01x | 26/30 |
| san_parse_all | 0.98x | 1.81x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.30x | N/A | 2.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.28x | 1.51x | 4.16x | 1.23x | 1.24x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 0.86x | 1.08x | 66.00x | 111.10x | 151.31x | 120.60x | 5/5 |
| chess960_materialize | 0.98x | 1.67x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.80x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.19x | 1.62x | 1.33x | 1.90x | 2.09x | 10/10 |
| corpus_materialize | 0.99x | 1.58x | 1.51x | 1.22x | 1.90x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.93x | 2.08x | 2.84x | 1.46x | 10/10 |
| replay_trusted_native | 0.92x | 0.58x | 1.52x | 1.85x | 2.04x | 2.09x | 8/10 |
| replay_checked_native | 0.95x | 2.19x | 1.08x | 7.17x | 6.45x | 8.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
