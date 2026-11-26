| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.39x | 1.38x | 1.50x | 1.98x | 1.64x | 30/30 |
| materialize | 1.11x | 1.91x | 1.47x | 1.03x | 2.24x | 1.32x | 27/30 |
| perft_bulk_d4 | 1.10x | 3.73x | 1.43x | 1.18x | 1.97x | 1.69x | 28/30 |
| perft_full_d3 | 1.01x | 1.55x | 1.66x | 1.27x | 2.05x | 1.93x | 30/30 |
| fen_parse | 0.96x | 1.19x | 1.28x | 1.27x | 1.55x | 1.45x | 29/30 |
| fen_write_native_ep | 0.98x | 3.97x | 8.74x | 1.85x | 26.97x | 2.15x | 30/30 |
| snapshot | 1.24x | 1.69x | 1.10x | 2.53x | 1.13x | 1.09x | 28/30 |
| san_parse_all | 1.06x | 1.66x | N/A | 3.77x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.13x | N/A | 2.59x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.07x | 1.20x | 5.82x | 1.15x | 1.34x | 27/30 |
| make_unmake_full | 1.11x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.19x | 71.02x | 149.95x | 165.09x | 148.20x | 5/5 |
| chess960_materialize | 1.15x | 1.62x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.19x | 2.68x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.31x | 38.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.12x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.32x | 19.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.12x | 5.20x | 1.72x | 0.96x | 1.97x | 2.08x | 9/10 |
| corpus_materialize | 1.13x | 1.43x | 1.29x | 1.05x | 1.76x | 1.14x | 9/10 |
| corpus_uci_sorted | 1.23x | 1.86x | 2.64x | 1.95x | 2.51x | 1.47x | 10/10 |
| replay_trusted_native | 1.53x | 0.66x | 1.66x | 1.70x | 1.81x | 1.78x | 8/10 |
| replay_checked_native | 1.06x | 2.33x | 1.12x | 8.35x | 7.39x | 10.57x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
