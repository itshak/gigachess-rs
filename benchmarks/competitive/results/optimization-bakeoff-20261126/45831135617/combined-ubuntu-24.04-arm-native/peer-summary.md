| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 3.68x | 1.41x | 1.52x | 1.83x | 1.83x | 30/30 |
| materialize | 0.98x | 1.80x | 1.41x | 1.24x | 1.99x | 1.38x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.33x | 1.36x | 1.24x | 1.73x | 1.78x | 30/30 |
| perft_full_d3 | 0.93x | 1.09x | 1.18x | 0.97x | 1.44x | 1.37x | 25/30 |
| fen_parse | 1.01x | 1.25x | 1.33x | 1.27x | 1.75x | 1.42x | 30/30 |
| fen_write_native_ep | 0.95x | 4.41x | 8.85x | 1.48x | 28.29x | 2.26x | 30/30 |
| snapshot | 1.07x | 1.13x | 1.18x | 1.54x | 1.18x | 1.02x | 30/30 |
| san_parse_all | 0.97x | 1.80x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 11.47x | N/A | 2.47x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.22x | 1.46x | 3.96x | 1.18x | 1.18x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.50x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.24x | 71.27x | 119.70x | 159.93x | 128.35x | 5/5 |
| chess960_materialize | 0.98x | 1.55x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 26.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 3.76x | 1.45x | 1.17x | 1.69x | 1.90x | 10/10 |
| corpus_materialize | 0.98x | 1.50x | 1.43x | 1.15x | 1.79x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.14x | 2.95x | 2.12x | 2.84x | 1.48x | 10/10 |
| replay_trusted_native | 0.99x | 0.60x | 1.57x | 1.94x | 2.11x | 2.19x | 8/10 |
| replay_checked_native | 0.98x | 2.11x | 1.03x | 6.91x | 6.15x | 8.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
