| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 2.73x | 1.43x | 1.30x | 1.90x | 1.26x | 29/30 |
| materialize | 1.00x | 1.97x | 1.66x | 1.16x | 2.23x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.94x | 2.55x | 1.42x | 1.11x | 1.75x | 1.51x | 30/30 |
| perft_full_d3 | 0.95x | 1.15x | 1.44x | 1.09x | 1.66x | 1.52x | 28/30 |
| fen_parse | 1.18x | 1.61x | 1.59x | 1.27x | 2.01x | 1.41x | 30/30 |
| fen_write_native_ep | 0.91x | 2.39x | 8.51x | 1.60x | 24.54x | 1.85x | 30/30 |
| snapshot | 1.00x | 1.71x | 1.21x | 2.84x | 1.13x | 0.98x | 24/30 |
| san_parse_all | 0.99x | 2.00x | N/A | 3.82x | N/A | N/A | 12/12 |
| san_render_native | 0.80x | 8.07x | N/A | 1.88x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.95x | 1.29x | 4.94x | 1.23x | 1.27x | 30/30 |
| make_unmake_full | 0.79x | N/A | N/A | 1.10x | N/A | N/A | 6/6 |
| startpos | 1.27x | 1.69x | 93.95x | 130.06x | 211.41x | 149.26x | 5/5 |
| chess960_materialize | 0.95x | 1.54x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 1.93x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 22.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 20.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 2.69x | 1.56x | 1.09x | 1.68x | 1.61x | 9/10 |
| corpus_materialize | 0.96x | 1.71x | 1.64x | 1.18x | 2.69x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.08x | 2.34x | 3.32x | 2.53x | 3.28x | 1.85x | 10/10 |
| replay_trusted_native | 1.00x | 0.95x | 1.95x | 2.05x | 3.06x | 2.83x | 9/10 |
| replay_checked_native | 0.93x | 2.01x | 1.01x | 5.35x | 6.75x | 6.96x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
