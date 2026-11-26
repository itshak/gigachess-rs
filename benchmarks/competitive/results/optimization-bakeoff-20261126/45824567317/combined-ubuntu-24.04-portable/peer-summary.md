| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.73x | 1.55x | 1.35x | 1.78x | 1.28x | 29/30 |
| materialize | 0.95x | 2.15x | 1.66x | 1.74x | 2.52x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.71x | 1.45x | 1.07x | 1.78x | 1.61x | 29/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.75x | 1.56x | 2.05x | 1.74x | 30/30 |
| fen_parse | 1.02x | 1.57x | 1.39x | 1.15x | 1.48x | 1.16x | 30/30 |
| fen_write_native_ep | 1.01x | 2.97x | 9.95x | 2.06x | 27.69x | 2.30x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.98x | 2.01x | N/A | 4.10x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.63x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.15x | 1.38x | 4.11x | 1.33x | 1.52x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 63.40x | 100.93x | 124.97x | 99.76x | 5/5 |
| chess960_materialize | 1.02x | 1.62x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.99x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.02x | 19.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 3.61x | 1.82x | 0.99x | 1.81x | 2.47x | 9/10 |
| corpus_materialize | 0.99x | 1.43x | 1.34x | 1.09x | 1.75x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.91x | 2.91x | 2.16x | 2.83x | 1.52x | 10/10 |
| replay_trusted_native | 1.01x | 0.88x | 1.68x | 1.62x | 2.21x | 2.04x | 9/10 |
| replay_checked_native | 1.12x | 2.03x | 1.20x | 9.02x | 8.89x | 9.99x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
