| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.09x | 4.92x | 1.69x | 2.01x | 2.27x | 2.21x | 30/30 |
| materialize | 1.04x | 1.95x | 1.47x | 1.05x | 2.24x | 1.33x | 28/30 |
| perft_bulk_d4 | 1.16x | 4.33x | 1.69x | 1.31x | 2.15x | 1.96x | 29/30 |
| perft_full_d3 | 1.23x | 1.70x | 1.92x | 1.51x | 2.18x | 2.22x | 30/30 |
| fen_parse | 1.04x | 1.26x | 1.37x | 1.32x | 1.59x | 1.43x | 30/30 |
| fen_write_native_ep | 1.10x | 4.20x | 8.34x | 1.71x | 26.32x | 2.21x | 30/30 |
| snapshot | 0.98x | 1.51x | 0.95x | 2.30x | 0.96x | 0.94x | 18/30 |
| san_parse_all | 1.06x | 1.69x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 12.46x | N/A | 2.87x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.19x | 1.23x | 6.02x | 1.18x | 1.32x | 30/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.61x | 67.48x | 153.65x | 178.98x | 156.19x | 5/5 |
| chess960_materialize | 0.65x | 1.54x | 1.15x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 3.13x | 1.68x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 43.53x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.46x | 1.43x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 19.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 5.86x | 2.28x | 1.19x | 2.18x | 3.09x | 10/10 |
| corpus_materialize | 1.00x | 1.66x | 1.75x | 1.44x | 2.06x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.68x | 1.79x | 1.72x | 1.35x | 1.60x | 1.10x | 10/10 |
| replay_trusted_native | 0.98x | 0.62x | 1.80x | 1.66x | 1.75x | 1.84x | 8/10 |
| replay_checked_native | 1.10x | 2.79x | 1.45x | 10.23x | 8.95x | 10.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
