| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.45x | 1.46x | 1.59x | 2.05x | 1.79x | 30/30 |
| materialize | 1.10x | 2.00x | 1.53x | 1.08x | 2.34x | 1.46x | 29/30 |
| perft_bulk_d4 | 1.11x | 3.46x | 1.31x | 1.04x | 1.65x | 1.58x | 26/30 |
| perft_full_d3 | 1.12x | 1.50x | 1.67x | 1.26x | 1.84x | 1.87x | 30/30 |
| fen_parse | 1.17x | 1.27x | 1.36x | 1.39x | 1.73x | 1.57x | 30/30 |
| fen_write_native_ep | 1.13x | 4.25x | 9.12x | 1.75x | 27.78x | 2.20x | 30/30 |
| snapshot | 1.21x | 1.81x | 1.13x | 2.33x | 1.13x | 1.04x | 26/30 |
| san_parse_all | 1.07x | 1.67x | N/A | 3.52x | N/A | N/A | 12/12 |
| san_render_native | 1.13x | 11.88x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.01x | 1.12x | 5.31x | 1.07x | 1.20x | 26/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.02x | 1.20x | 66.75x | 134.12x | 173.70x | 136.88x | 5/5 |
| chess960_materialize | 0.91x | 1.61x | 1.42x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.79x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.97x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.60x | 1.97x | 0.97x | 2.00x | 2.03x | 9/10 |
| corpus_materialize | 1.28x | 1.40x | 1.34x | 1.06x | 1.80x | 1.14x | 10/10 |
| corpus_uci_sorted | 1.08x | 1.85x | 2.69x | 1.94x | 2.56x | 1.41x | 10/10 |
| replay_trusted_native | 0.98x | 0.58x | 1.43x | 1.60x | 1.74x | 1.63x | 8/10 |
| replay_checked_native | 1.19x | 2.20x | 1.04x | 7.80x | 6.84x | 8.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
