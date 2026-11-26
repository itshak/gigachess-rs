| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 5.00x | 1.74x | 1.59x | 2.33x | 2.04x | 30/30 |
| materialize | 0.99x | 2.06x | 1.43x | 1.23x | 2.35x | 1.64x | 30/30 |
| perft_bulk_d4 | 0.91x | 3.86x | 1.45x | 1.24x | 2.09x | 1.84x | 29/30 |
| perft_full_d3 | 0.87x | 0.97x | 1.03x | 0.92x | 1.35x | 1.31x | 19/30 |
| fen_parse | 1.01x | 1.19x | 1.21x | 1.20x | 1.97x | 1.28x | 30/30 |
| fen_write_native_ep | 1.00x | 4.10x | 8.20x | 1.54x | 31.50x | 2.02x | 30/30 |
| snapshot | 0.96x | 1.78x | 1.01x | 3.89x | 0.96x | 0.96x | 19/30 |
| san_parse_all | 1.00x | 2.18x | N/A | 5.57x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.80x | N/A | 3.64x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.27x | 1.40x | 9.25x | 1.48x | 1.74x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 79.39x | 157.70x | 232.26x | 171.44x | 5/5 |
| chess960_materialize | 1.09x | 1.65x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.88x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 27.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 21.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.37x | 1.83x | 1.37x | 2.49x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.55x | 1.24x | 1.15x | 1.98x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.75x | 2.16x | 1.79x | 2.12x | 1.39x | 10/10 |
| replay_trusted_native | 0.99x | 0.83x | 1.57x | 2.03x | 2.55x | 2.98x | 9/10 |
| replay_checked_native | 0.97x | 2.18x | 0.99x | 7.65x | 7.39x | 8.74x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
