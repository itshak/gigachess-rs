| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.77x | 1.78x | 1.73x | 2.53x | 1.87x | 30/30 |
| materialize | 1.14x | 2.14x | 1.69x | 1.17x | 2.36x | 1.54x | 29/30 |
| perft_bulk_d4 | 1.05x | 3.89x | 1.56x | 1.32x | 1.97x | 1.75x | 29/30 |
| perft_full_d3 | 1.02x | 1.36x | 1.48x | 1.14x | 1.67x | 1.79x | 29/30 |
| fen_parse | 1.10x | 1.28x | 1.43x | 1.36x | 1.67x | 1.45x | 30/30 |
| fen_write_native_ep | 1.06x | 4.04x | 9.23x | 1.81x | 27.24x | 2.39x | 30/30 |
| snapshot | 0.93x | 1.50x | 1.02x | 2.23x | 1.17x | 0.93x | 22/30 |
| san_parse_all | 1.03x | 1.79x | N/A | 3.84x | N/A | N/A | 12/12 |
| san_render_native | 1.07x | 11.32x | N/A | 2.66x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.14x | 1.19x | 5.55x | 1.15x | 1.40x | 28/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.43x | 1.28x | 111.69x | 156.72x | 183.10x | 159.33x | 5/5 |
| chess960_materialize | 0.92x | 1.88x | 1.66x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.63x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.82x | 37.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.81x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 20.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 4.90x | 1.92x | 1.34x | 2.21x | 2.28x | 10/10 |
| corpus_materialize | 1.16x | 1.43x | 1.42x | 1.09x | 1.88x | 1.39x | 9/10 |
| corpus_uci_sorted | 1.03x | 2.04x | 2.48x | 1.89x | 2.49x | 1.53x | 10/10 |
| replay_trusted_native | 0.99x | 0.67x | 1.76x | 1.61x | 2.13x | 1.94x | 8/10 |
| replay_checked_native | 0.76x | 2.59x | 1.24x | 8.39x | 9.30x | 9.29x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
