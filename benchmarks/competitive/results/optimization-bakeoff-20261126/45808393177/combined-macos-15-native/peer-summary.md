| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.15x | 4.27x | 1.39x | 1.48x | 1.93x | 1.74x | 29/30 |
| materialize | 1.32x | 1.92x | 1.46x | 1.14x | 2.29x | 1.46x | 28/30 |
| perft_bulk_d4 | 0.93x | 3.53x | 1.48x | 1.15x | 1.73x | 1.66x | 28/30 |
| perft_full_d3 | 1.03x | 1.59x | 1.77x | 1.28x | 1.97x | 1.98x | 30/30 |
| fen_parse | 1.08x | 1.29x | 1.33x | 1.29x | 1.69x | 1.33x | 30/30 |
| fen_write_native_ep | 1.02x | 4.58x | 9.65x | 1.80x | 28.47x | 2.25x | 30/30 |
| snapshot | 1.08x | 1.81x | 1.14x | 2.42x | 1.10x | 1.06x | 28/30 |
| san_parse_all | 0.95x | 1.70x | N/A | 3.42x | N/A | N/A | 12/12 |
| san_render_native | 1.15x | 11.62x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.31x | 1.27x | 1.35x | 6.63x | 1.28x | 1.55x | 30/30 |
| make_unmake_full | 1.22x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.22x | 71.44x | 141.04x | 178.25x | 131.27x | 5/5 |
| chess960_materialize | 1.11x | 1.52x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.63x | 1.19x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 38.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.12x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.11x | 19.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.46x | 1.53x | 0.92x | 1.84x | 1.88x | 8/10 |
| corpus_materialize | 1.00x | 1.41x | 1.36x | 1.12x | 1.87x | 1.22x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.91x | 2.65x | 1.95x | 2.60x | 1.49x | 10/10 |
| replay_trusted_native | 1.34x | 0.70x | 1.76x | 1.85x | 1.99x | 2.11x | 8/10 |
| replay_checked_native | 1.13x | 2.48x | 0.98x | 8.37x | 6.97x | 8.58x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
