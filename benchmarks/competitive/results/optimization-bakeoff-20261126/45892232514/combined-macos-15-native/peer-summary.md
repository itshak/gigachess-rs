| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.91x | 4.57x | 1.63x | 1.70x | 2.29x | 1.95x | 30/30 |
| materialize | 0.98x | 2.13x | 1.72x | 1.10x | 2.19x | 1.56x | 28/30 |
| perft_bulk_d4 | 0.76x | 3.20x | 1.23x | 1.00x | 1.66x | 1.47x | 25/30 |
| perft_full_d3 | 0.69x | 1.12x | 1.23x | 0.97x | 1.38x | 1.54x | 26/30 |
| fen_parse | 0.89x | 1.29x | 1.31x | 1.29x | 1.61x | 1.42x | 30/30 |
| fen_write_native_ep | 0.89x | 3.81x | 8.20x | 1.49x | 25.18x | 1.92x | 30/30 |
| snapshot | 1.09x | 1.71x | 1.19x | 2.95x | 1.25x | 1.00x | 23/30 |
| san_parse_all | 0.99x | 1.82x | N/A | 3.68x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 12.79x | N/A | 3.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.83x | 1.09x | 1.13x | 5.37x | 1.05x | 1.18x | 22/30 |
| make_unmake_full | 0.76x | N/A | N/A | 1.03x | N/A | N/A | 2/6 |
| startpos | 2.07x | 2.43x | 167.92x | 317.98x | 394.29x | 322.99x | 5/5 |
| chess960_materialize | 1.02x | 1.69x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.81x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.25x | 41.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 0.95x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 21.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 5.09x | 1.83x | 1.26x | 2.22x | 2.82x | 10/10 |
| corpus_materialize | 1.05x | 1.31x | 1.26x | 1.02x | 1.55x | 1.09x | 8/10 |
| corpus_uci_sorted | 1.01x | 1.82x | 2.69x | 1.88x | 2.40x | 1.30x | 10/10 |
| replay_trusted_native | 1.06x | 0.61x | 1.57x | 1.51x | 1.88x | 1.76x | 8/10 |
| replay_checked_native | 0.96x | 2.29x | 1.08x | 8.04x | 7.03x | 8.32x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
