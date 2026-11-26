| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 5.06x | 1.59x | 1.66x | 2.37x | 2.13x | 30/30 |
| materialize | 1.02x | 2.08x | 1.56x | 1.10x | 2.31x | 1.42x | 29/30 |
| perft_bulk_d4 | 0.92x | 4.13x | 1.53x | 1.21x | 1.98x | 1.77x | 29/30 |
| perft_full_d3 | 0.74x | 1.25x | 1.31x | 1.03x | 1.49x | 1.51x | 28/30 |
| fen_parse | 0.87x | 1.15x | 1.21x | 1.20x | 1.51x | 1.22x | 29/30 |
| fen_write_native_ep | 1.00x | 4.19x | 8.57x | 1.72x | 27.10x | 2.21x | 30/30 |
| snapshot | 1.01x | 1.77x | 1.15x | 2.73x | 1.26x | 1.01x | 26/30 |
| san_parse_all | 1.01x | 1.81x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.28x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.21x | 1.32x | 5.93x | 1.24x | 1.32x | 29/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.16x | N/A | N/A | 5/6 |
| startpos | 0.69x | 0.99x | 47.86x | 104.07x | 107.53x | 95.37x | 4/5 |
| chess960_materialize | 1.13x | 1.69x | 1.45x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 3.35x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 43.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 21.72x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 6.39x | 1.98x | 1.33x | 2.25x | 2.59x | 10/10 |
| corpus_materialize | 0.99x | 1.47x | 1.46x | 1.10x | 1.83x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.78x | 2.84x | 1.88x | 2.48x | 1.45x | 10/10 |
| replay_trusted_native | 1.01x | 0.68x | 1.85x | 1.78x | 2.26x | 1.96x | 8/10 |
| replay_checked_native | 0.76x | 2.28x | 0.93x | 7.53x | 5.99x | 8.11x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
