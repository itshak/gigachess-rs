| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.71x | 1.66x | 1.60x | 2.49x | 1.94x | 30/30 |
| materialize | 0.95x | 2.14x | 1.68x | 1.14x | 2.15x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.97x | 4.25x | 1.64x | 1.31x | 2.52x | 2.22x | 30/30 |
| perft_full_d3 | 0.84x | 1.47x | 1.43x | 1.00x | 1.86x | 1.59x | 27/30 |
| fen_parse | 0.83x | 1.18x | 1.32x | 0.97x | 1.66x | 1.17x | 25/30 |
| fen_write_native_ep | 1.00x | 4.01x | 9.34x | 1.85x | 28.05x | 2.10x | 30/30 |
| snapshot | 0.73x | 1.51x | 1.08x | 2.72x | 1.11x | 1.00x | 26/30 |
| san_parse_all | 0.88x | 1.98x | N/A | 3.41x | N/A | N/A | 12/12 |
| san_render_native | 0.95x | 9.82x | N/A | 1.99x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 1.87x | 1.22x | 4.52x | 1.39x | 1.61x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.75x | 0.98x | 73.62x | 104.57x | 165.96x | 123.40x | 4/5 |
| chess960_materialize | 0.89x | 1.73x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.91x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 20.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.20x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.38x | 1.74x | 1.27x | 2.31x | 2.01x | 9/10 |
| corpus_materialize | 0.97x | 1.88x | 1.71x | 1.24x | 2.78x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 3.13x | 2.34x | 2.92x | 1.66x | 10/10 |
| replay_trusted_native | 0.86x | 0.97x | 1.57x | 1.84x | 3.33x | 4.16x | 9/10 |
| replay_checked_native | 0.92x | 2.55x | 0.94x | 5.64x | 6.58x | 7.66x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
