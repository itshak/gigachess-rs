| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.99x | 1.72x | 1.49x | 2.27x | 1.78x | 30/30 |
| materialize | 1.01x | 2.06x | 1.64x | 1.60x | 2.57x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.83x | 3.24x | 1.30x | 0.98x | 1.95x | 1.56x | 26/30 |
| perft_full_d3 | 0.72x | 1.05x | 1.23x | 1.14x | 1.49x | 1.30x | 26/30 |
| fen_parse | 0.91x | 1.22x | 1.21x | 0.96x | 1.31x | 1.11x | 23/30 |
| fen_write_native_ep | 0.95x | 4.39x | 9.53x | 2.01x | 27.70x | 2.51x | 30/30 |
| snapshot | 1.22x | 2.12x | 1.32x | 3.73x | 1.32x | 1.41x | 30/30 |
| san_parse_all | 0.95x | 1.90x | N/A | 4.35x | N/A | N/A | 12/12 |
| san_render_native | 0.88x | 8.22x | N/A | 1.98x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.56x | 1.02x | 2.94x | 0.95x | 1.22x | 23/30 |
| make_unmake_full | 0.69x | N/A | N/A | 0.76x | N/A | N/A | 0/6 |
| startpos | 0.84x | 1.15x | 71.78x | 123.19x | 160.14x | 131.66x | 5/5 |
| chess960_materialize | 0.92x | 1.52x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.77x | 1.89x | 1.08x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 39.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 18.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.36x | 2.41x | 1.35x | 2.90x | 2.55x | 10/10 |
| corpus_materialize | 1.03x | 1.46x | 1.34x | 1.10x | 1.76x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.31x | 2.91x | 2.25x | 2.93x | 1.54x | 10/10 |
| replay_trusted_native | 0.95x | 0.69x | 1.23x | 1.21x | 1.69x | 1.98x | 8/10 |
| replay_checked_native | 0.71x | 1.48x | 0.79x | 6.03x | 5.84x | 7.22x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
