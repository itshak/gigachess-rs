| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.84x | 4.86x | 1.62x | 1.62x | 2.35x | 2.12x | 29/30 |
| materialize | 0.93x | 2.30x | 1.92x | 1.05x | 2.34x | 1.44x | 26/30 |
| perft_bulk_d4 | 1.08x | 4.56x | 1.62x | 1.26x | 1.92x | 1.83x | 30/30 |
| perft_full_d3 | 1.12x | 1.36x | 1.48x | 1.26x | 1.77x | 1.85x | 29/30 |
| fen_parse | 0.99x | 1.26x | 1.52x | 1.25x | 1.88x | 1.46x | 30/30 |
| fen_write_native_ep | 1.04x | 4.26x | 12.98x | 1.77x | 28.33x | 2.26x | 30/30 |
| snapshot | 0.85x | 1.76x | 0.98x | 2.36x | 0.98x | 0.97x | 22/30 |
| san_parse_all | 0.93x | 1.77x | N/A | 3.95x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.41x | N/A | 2.58x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.07x | 1.18x | 5.90x | 1.22x | 1.32x | 27/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.20x | 67.34x | 133.93x | 182.59x | 137.16x | 5/5 |
| chess960_materialize | 1.01x | 1.75x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.16x | 3.54x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 35.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 18.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 5.94x | 1.82x | 1.30x | 2.34x | 2.55x | 10/10 |
| corpus_materialize | 1.26x | 1.40x | 1.28x | 1.17x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.84x | 2.59x | 1.80x | 2.40x | 1.34x | 10/10 |
| replay_trusted_native | 1.28x | 0.70x | 1.79x | 1.92x | 1.85x | 2.15x | 8/10 |
| replay_checked_native | 1.25x | 2.76x | 1.17x | 10.95x | 9.55x | 12.12x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
