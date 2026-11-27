| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 5.06x | 1.86x | 1.69x | 2.24x | 1.92x | 30/30 |
| materialize | 1.11x | 2.09x | 1.64x | 1.25x | 2.43x | 1.58x | 29/30 |
| perft_bulk_d4 | 0.99x | 4.03x | 1.53x | 1.27x | 1.80x | 1.70x | 29/30 |
| perft_full_d3 | 1.08x | 1.61x | 2.00x | 1.51x | 2.15x | 2.28x | 30/30 |
| fen_parse | 1.07x | 1.33x | 1.39x | 1.38x | 1.68x | 1.49x | 30/30 |
| fen_write_native_ep | 1.06x | 4.57x | 10.36x | 1.88x | 30.08x | 2.41x | 30/30 |
| snapshot | 1.05x | 1.64x | 1.21x | 2.55x | 1.05x | 0.98x | 23/30 |
| san_parse_all | 0.95x | 1.70x | N/A | 3.69x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 13.37x | N/A | 3.05x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.11x | 1.22x | 6.03x | 1.22x | 1.26x | 28/30 |
| make_unmake_full | 1.13x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.19x | 1.11x | 83.41x | 189.90x | 217.54x | 210.45x | 5/5 |
| chess960_materialize | 0.98x | 2.13x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 3.06x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.83x | 38.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.84x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 21.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 5.39x | 1.97x | 1.21x | 2.18x | 2.48x | 9/10 |
| corpus_materialize | 0.89x | 1.63x | 1.55x | 1.24x | 2.09x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.89x | 2.42x | 2.03x | 2.64x | 1.47x | 10/10 |
| replay_trusted_native | 1.08x | 0.70x | 1.71x | 1.85x | 1.88x | 2.10x | 8/10 |
| replay_checked_native | 1.08x | 3.08x | 1.46x | 10.79x | 9.90x | 11.62x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
