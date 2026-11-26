| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.85x | 4.65x | 1.46x | 1.66x | 2.15x | 1.88x | 30/30 |
| materialize | 0.97x | 2.25x | 1.64x | 1.18x | 2.19x | 1.51x | 29/30 |
| perft_bulk_d4 | 1.03x | 3.97x | 1.52x | 1.19x | 1.99x | 1.97x | 30/30 |
| perft_full_d3 | 1.04x | 1.51x | 1.71x | 1.29x | 1.94x | 1.99x | 30/30 |
| fen_parse | 1.05x | 1.28x | 1.41x | 1.39x | 1.92x | 1.40x | 30/30 |
| fen_write_native_ep | 1.01x | 4.03x | 12.82x | 1.66x | 25.78x | 2.10x | 30/30 |
| snapshot | 1.02x | 1.80x | 1.11x | 2.68x | 1.34x | 1.07x | 27/30 |
| san_parse_all | 0.97x | 1.84x | N/A | 3.64x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 11.74x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.10x | 1.35x | 6.05x | 1.22x | 1.41x | 29/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 0.79x | 0.93x | 64.09x | 119.65x | 181.51x | 134.49x | 4/5 |
| chess960_materialize | 1.01x | 1.73x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.85x | 2.77x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 38.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.84x | 0.83x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.05x | 20.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 6.29x | 2.38x | 1.58x | 2.58x | 3.01x | 10/10 |
| corpus_materialize | 0.85x | 1.35x | 1.32x | 0.98x | 1.66x | 1.14x | 9/10 |
| corpus_uci_sorted | 1.17x | 2.13x | 2.64x | 1.64x | 2.36x | 1.33x | 10/10 |
| replay_trusted_native | 1.10x | 0.71x | 1.62x | 1.95x | 1.83x | 1.86x | 8/10 |
| replay_checked_native | 1.09x | 2.47x | 1.16x | 8.99x | 7.73x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
