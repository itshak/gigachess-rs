| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.39x | 1.88x | 1.58x | 2.43x | 1.94x | 30/30 |
| materialize | 0.93x | 2.06x | 1.55x | 1.70x | 2.39x | 1.83x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.93x | 1.62x | 1.25x | 2.42x | 1.95x | 29/30 |
| perft_full_d3 | 1.01x | 1.36x | 1.57x | 1.49x | 1.98x | 1.74x | 30/30 |
| fen_parse | 0.99x | 1.39x | 1.57x | 1.13x | 1.45x | 1.25x | 30/30 |
| fen_write_native_ep | 0.99x | 4.65x | 9.91x | 1.89x | 27.38x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.16x | 4.30x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.90x | 1.72x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.16x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.34x | 4.03x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.88x | 1.11x | 87.40x | 166.31x | 194.06x | 178.00x | 5/5 |
| chess960_materialize | 0.99x | 1.75x | 1.74x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.99x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.23x | 23.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.22x | 6.42x | 2.42x | 1.33x | 2.94x | 2.71x | 10/10 |
| corpus_materialize | 1.02x | 1.41x | 1.30x | 1.08x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.28x | 2.98x | 2.28x | 2.88x | 1.58x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.57x | 1.52x | 2.12x | 2.79x | 9/10 |
| replay_checked_native | 1.02x | 2.23x | 1.24x | 9.32x | 8.97x | 10.94x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
