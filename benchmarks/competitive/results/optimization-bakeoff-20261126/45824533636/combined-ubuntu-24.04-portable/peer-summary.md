| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.88x | 1.51x | 1.29x | 1.84x | 1.29x | 29/30 |
| materialize | 1.01x | 2.22x | 1.75x | 1.28x | 2.62x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.95x | 2.80x | 1.43x | 1.09x | 1.76x | 1.62x | 28/30 |
| perft_full_d3 | 0.98x | 1.40x | 1.71x | 1.28x | 1.90x | 1.75x | 30/30 |
| fen_parse | 1.01x | 1.67x | 1.64x | 1.26x | 2.03x | 1.35x | 30/30 |
| fen_write_native_ep | 0.98x | 2.83x | 8.43x | 1.52x | 25.28x | 1.89x | 30/30 |
| snapshot | 1.00x | 1.62x | 1.16x | 2.64x | 1.16x | 1.00x | 27/30 |
| san_parse_all | 0.99x | 1.76x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 9.89x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.51x | 1.37x | 4.95x | 1.14x | 1.25x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.28x | 58.74x | 79.84x | 117.55x | 80.43x | 5/5 |
| chess960_materialize | 0.98x | 1.74x | 1.51x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.12x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 33.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.63x | 12.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.06x | 1.68x | 1.03x | 1.80x | 2.66x | 9/10 |
| corpus_materialize | 1.02x | 1.67x | 1.49x | 1.15x | 2.14x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.00x | 2.78x | 2.24x | 2.77x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.66x | 1.49x | 1.53x | 2.03x | 1.86x | 8/10 |
| replay_checked_native | 1.02x | 2.20x | 1.22x | 8.31x | 8.33x | 9.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
