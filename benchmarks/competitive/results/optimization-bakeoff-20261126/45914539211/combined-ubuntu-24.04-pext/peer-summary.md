| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.18x | 1.76x | 1.78x | 2.78x | 2.11x | 30/30 |
| materialize | 1.01x | 2.02x | 1.66x | 1.28x | 2.33x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.26x | 1.66x | 1.48x | 2.56x | 2.08x | 30/30 |
| perft_full_d3 | 1.22x | 1.63x | 1.88x | 1.64x | 2.40x | 2.22x | 30/30 |
| fen_parse | 0.95x | 1.37x | 1.35x | 1.14x | 2.10x | 1.36x | 30/30 |
| fen_write_native_ep | 1.01x | 4.34x | 9.92x | 1.73x | 28.48x | 2.06x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.25x | 3.28x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.10x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.34x | N/A | 2.20x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.41x | 1.38x | 4.76x | 1.52x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.27x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.48x | 133.46x | 222.94x | 153.25x | 5/5 |
| chess960_materialize | 1.02x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.86x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 25.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.12x | 1.99x | 1.55x | 2.99x | 2.55x | 10/10 |
| corpus_materialize | 0.99x | 1.64x | 1.58x | 1.18x | 2.06x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.47x | 3.22x | 2.44x | 3.24x | 1.79x | 10/10 |
| replay_trusted_native | 0.96x | 0.87x | 1.63x | 1.88x | 2.53x | 2.84x | 9/10 |
| replay_checked_native | 0.97x | 2.49x | 1.09x | 8.22x | 8.32x | 9.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
