| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.41x | 1.46x | 1.66x | 2.04x | 1.98x | 27/30 |
| materialize | 0.93x | 1.85x | 1.46x | 1.04x | 2.09x | 1.31x | 25/30 |
| perft_bulk_d4 | 1.11x | 4.32x | 1.67x | 1.35x | 2.17x | 2.20x | 30/30 |
| perft_full_d3 | 1.07x | 1.73x | 2.01x | 1.45x | 2.13x | 2.16x | 30/30 |
| fen_parse | 0.99x | 1.29x | 1.33x | 1.43x | 1.68x | 1.53x | 30/30 |
| fen_write_native_ep | 0.97x | 4.24x | 8.98x | 1.78x | 28.01x | 2.17x | 30/30 |
| snapshot | 0.86x | 1.62x | 1.25x | 2.24x | 1.02x | 0.87x | 24/30 |
| san_parse_all | 0.90x | 1.70x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.92x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.19x | 1.34x | 6.37x | 1.22x | 1.41x | 29/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.07x | 1.16x | 65.73x | 135.04x | 157.92x | 141.00x | 5/5 |
| chess960_materialize | 1.01x | 1.77x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 3.43x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 41.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.80x | 0.84x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.90x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 6.61x | 1.99x | 1.25x | 2.35x | 2.55x | 10/10 |
| corpus_materialize | 0.95x | 1.52x | 1.41x | 1.16x | 1.86x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.92x | 1.95x | 2.81x | 2.10x | 2.53x | 1.52x | 10/10 |
| replay_trusted_native | 0.83x | 0.68x | 1.77x | 1.39x | 1.85x | 1.79x | 7/10 |
| replay_checked_native | 0.94x | 2.98x | 1.45x | 11.53x | 10.68x | 13.05x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
