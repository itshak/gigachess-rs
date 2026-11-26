| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.50x | 1.54x | 1.58x | 2.08x | 1.75x | 30/30 |
| materialize | 1.02x | 1.94x | 1.50x | 1.00x | 2.22x | 1.29x | 27/30 |
| perft_bulk_d4 | 1.02x | 4.45x | 1.63x | 1.32x | 2.08x | 1.91x | 30/30 |
| perft_full_d3 | 1.02x | 1.57x | 1.75x | 1.33x | 1.91x | 2.07x | 30/30 |
| fen_parse | 1.09x | 1.25x | 1.36x | 1.36x | 1.64x | 1.53x | 30/30 |
| fen_write_native_ep | 1.00x | 4.49x | 9.65x | 1.81x | 30.56x | 2.27x | 30/30 |
| snapshot | 1.05x | 1.69x | 1.19x | 2.48x | 1.26x | 1.05x | 29/30 |
| san_parse_all | 0.99x | 1.94x | N/A | 3.73x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.36x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 0.95x | 1.05x | 4.99x | 0.98x | 1.11x | 19/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.59x | 75.82x | 149.14x | 160.59x | 144.64x | 5/5 |
| chess960_materialize | 0.97x | 1.62x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.90x | 3.01x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 35.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 0.93x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 6.30x | 2.19x | 1.44x | 2.60x | 3.10x | 10/10 |
| corpus_materialize | 0.82x | 1.50x | 1.31x | 1.00x | 1.80x | 1.24x | 9/10 |
| corpus_uci_sorted | 0.89x | 1.63x | 2.42x | 1.72x | 2.20x | 1.54x | 10/10 |
| replay_trusted_native | 0.81x | 0.44x | 1.09x | 1.10x | 1.19x | 1.22x | 7/10 |
| replay_checked_native | 1.17x | 2.34x | 1.17x | 8.36x | 6.99x | 8.83x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
