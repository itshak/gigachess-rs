| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.30x | 1.85x | 1.57x | 2.42x | 1.91x | 30/30 |
| materialize | 0.99x | 2.22x | 1.67x | 1.75x | 2.62x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.75x | 1.54x | 1.17x | 2.30x | 1.86x | 29/30 |
| perft_full_d3 | 1.01x | 1.61x | 1.55x | 1.49x | 1.99x | 1.68x | 30/30 |
| fen_parse | 0.94x | 1.32x | 1.25x | 1.08x | 1.39x | 1.19x | 30/30 |
| fen_write_native_ep | 1.00x | 5.04x | 10.40x | 1.88x | 29.50x | 2.51x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.93x | 1.78x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.07x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.02x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 88.60x | 170.35x | 193.54x | 175.90x | 5/5 |
| chess960_materialize | 0.92x | 1.62x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.66x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.26x | 1.97x | 1.09x | 2.41x | 2.22x | 9/10 |
| corpus_materialize | 1.01x | 1.40x | 1.29x | 1.07x | 1.70x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.11x | 2.23x | 3.03x | 1.56x | 10/10 |
| replay_trusted_native | 0.98x | 0.86x | 1.52x | 1.52x | 2.12x | 2.75x | 9/10 |
| replay_checked_native | 0.97x | 2.12x | 1.17x | 8.86x | 8.53x | 10.33x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
