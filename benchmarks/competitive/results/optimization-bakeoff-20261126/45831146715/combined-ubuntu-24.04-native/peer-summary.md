| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.91x | 1.67x | 1.42x | 2.24x | 1.71x | 30/30 |
| materialize | 1.01x | 2.13x | 1.67x | 1.65x | 2.59x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.26x | 1.33x | 1.01x | 2.01x | 1.60x | 26/30 |
| perft_full_d3 | 0.95x | 1.62x | 1.57x | 1.51x | 2.05x | 1.69x | 30/30 |
| fen_parse | 1.05x | 1.32x | 1.54x | 1.08x | 1.42x | 1.24x | 30/30 |
| fen_write_native_ep | 1.01x | 5.00x | 10.29x | 2.15x | 28.62x | 2.66x | 30/30 |
| snapshot | 0.99x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.98x | N/A | 4.60x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.84x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 1.89x | 1.22x | 3.61x | 1.18x | 1.54x | 30/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.03x | N/A | N/A | 4/6 |
| startpos | 0.89x | 1.00x | 80.13x | 139.65x | 170.21x | 155.93x | 5/5 |
| chess960_materialize | 0.96x | 1.57x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.29x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.17x | 41.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.91x | 20.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.22x | 1.59x | 0.88x | 1.94x | 1.74x | 8/10 |
| corpus_materialize | 1.00x | 1.36x | 1.27x | 1.04x | 1.67x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.23x | 2.93x | 2.18x | 2.86x | 1.54x | 10/10 |
| replay_trusted_native | 0.85x | 0.77x | 1.41x | 1.33x | 1.91x | 2.22x | 8/10 |
| replay_checked_native | 0.93x | 1.91x | 1.05x | 7.98x | 7.59x | 9.27x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
