| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.06x | 1.74x | 1.51x | 2.03x | 1.47x | 30/30 |
| materialize | 1.01x | 2.42x | 1.97x | 1.89x | 3.09x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.22x | 1.70x | 1.27x | 2.13x | 1.91x | 30/30 |
| perft_full_d3 | 1.15x | 1.57x | 1.82x | 1.64x | 2.20x | 1.86x | 30/30 |
| fen_parse | 1.03x | 1.51x | 1.49x | 1.24x | 1.66x | 1.20x | 30/30 |
| fen_write_native_ep | 1.00x | 3.15x | 9.95x | 2.07x | 26.58x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.24x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.33x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.26x | 1.48x | 4.20x | 1.38x | 1.57x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.36x | 57.01x | 110.06x | 130.66x | 95.05x | 5/5 |
| chess960_materialize | 1.05x | 1.81x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.15x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 41.64x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.61x | 1.53x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 22.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 4.87x | 2.42x | 1.34x | 2.49x | 3.35x | 10/10 |
| corpus_materialize | 1.01x | 1.52x | 1.42x | 1.14x | 1.84x | 1.26x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.97x | 2.93x | 2.19x | 2.86x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.90x | 1.69x | 1.70x | 2.33x | 2.09x | 9/10 |
| replay_checked_native | 1.03x | 2.08x | 1.20x | 9.07x | 8.88x | 10.15x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
