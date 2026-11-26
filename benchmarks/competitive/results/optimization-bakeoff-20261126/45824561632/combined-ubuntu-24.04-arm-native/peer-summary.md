| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.88x | 1.48x | 1.61x | 1.92x | 1.91x | 30/30 |
| materialize | 1.00x | 1.83x | 1.45x | 1.26x | 2.02x | 1.39x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.39x | 1.26x | 1.78x | 1.82x | 30/30 |
| perft_full_d3 | 0.92x | 1.08x | 1.17x | 0.98x | 1.42x | 1.37x | 24/30 |
| fen_parse | 1.04x | 1.29x | 1.38x | 1.31x | 1.83x | 1.46x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 9.25x | 1.58x | 29.88x | 2.49x | 30/30 |
| snapshot | 1.11x | 1.27x | 0.99x | 1.61x | 0.99x | 1.00x | 17/30 |
| san_parse_all | 0.97x | 1.77x | N/A | 4.13x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 11.52x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.18x | 1.38x | 3.85x | 1.13x | 1.14x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.53x | N/A | N/A | 6/6 |
| startpos | 0.83x | 1.02x | 63.15x | 107.40x | 146.30x | 118.40x | 5/5 |
| chess960_materialize | 0.96x | 1.51x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.48x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 26.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.10x | 20.34x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.96x | 1.52x | 1.22x | 1.78x | 1.96x | 10/10 |
| corpus_materialize | 0.98x | 1.50x | 1.43x | 1.15x | 1.79x | 1.28x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.91x | 2.08x | 2.81x | 1.45x | 10/10 |
| replay_trusted_native | 0.96x | 0.59x | 1.55x | 1.90x | 2.06x | 2.15x | 8/10 |
| replay_checked_native | 0.93x | 2.04x | 0.99x | 6.66x | 5.93x | 7.68x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
