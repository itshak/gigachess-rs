| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.13x | 1.56x | 1.69x | 2.02x | 2.02x | 30/30 |
| materialize | 0.98x | 1.91x | 1.49x | 1.30x | 2.09x | 1.45x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.27x | 1.33x | 1.21x | 1.69x | 1.73x | 30/30 |
| perft_full_d3 | 0.95x | 1.39x | 1.51x | 1.26x | 1.82x | 1.76x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.27x | 1.77x | 1.40x | 30/30 |
| fen_write_native_ep | 0.95x | 4.24x | 8.89x | 1.52x | 28.41x | 2.27x | 30/30 |
| snapshot | 1.34x | 1.55x | 1.33x | 1.95x | 1.33x | 1.22x | 30/30 |
| san_parse_all | 0.94x | 1.73x | N/A | 4.05x | N/A | N/A | 12/12 |
| san_render_native | 0.80x | 9.94x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 1.04x | 1.25x | 3.39x | 1.01x | 1.01x | 22/30 |
| make_unmake_full | 0.73x | N/A | N/A | 1.16x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.28x | 77.41x | 129.85x | 173.70x | 138.63x | 5/5 |
| chess960_materialize | 0.97x | 1.66x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.88x | 2.60x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 26.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 21.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 4.12x | 1.59x | 1.28x | 1.87x | 2.09x | 10/10 |
| corpus_materialize | 0.98x | 1.59x | 1.51x | 1.22x | 1.88x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.16x | 3.01x | 2.21x | 2.88x | 1.50x | 10/10 |
| replay_trusted_native | 0.81x | 0.52x | 1.33x | 1.64x | 1.79x | 1.84x | 8/10 |
| replay_checked_native | 0.88x | 2.02x | 1.00x | 6.65x | 5.99x | 7.77x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
