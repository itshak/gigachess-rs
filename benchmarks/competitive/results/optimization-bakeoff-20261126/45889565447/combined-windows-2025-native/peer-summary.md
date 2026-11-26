| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 4.61x | 1.70x | 1.64x | 2.46x | 2.14x | 30/30 |
| materialize | 1.00x | 1.98x | 1.32x | 1.11x | 2.28x | 1.53x | 29/30 |
| perft_bulk_d4 | 0.99x | 3.94x | 1.62x | 1.43x | 2.56x | 2.21x | 30/30 |
| perft_full_d3 | 1.33x | 1.96x | 2.05x | 1.69x | 2.74x | 2.43x | 30/30 |
| fen_parse | 1.06x | 1.54x | 1.66x | 1.59x | 2.84x | 1.63x | 30/30 |
| fen_write_native_ep | 1.01x | 4.16x | 8.24x | 1.62x | 31.56x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.79x | 1.26x | 4.11x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 0.99x | 2.20x | N/A | 4.66x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.31x | N/A | 3.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.95x | 1.36x | 8.23x | 1.53x | 1.76x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 91.07x | 138.63x | 214.29x | 146.22x | 4/5 |
| chess960_materialize | 1.00x | 1.62x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.86x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.08x | 1.64x | 1.21x | 2.17x | 1.97x | 9/10 |
| corpus_materialize | 1.00x | 1.75x | 1.36x | 1.19x | 2.79x | 1.49x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.86x | 2.38x | 1.97x | 2.28x | 1.49x | 10/10 |
| replay_trusted_native | 0.92x | 0.99x | 1.79x | 2.13x | 3.43x | 4.22x | 9/10 |
| replay_checked_native | 1.03x | 2.47x | 1.11x | 6.58x | 8.10x | 8.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
