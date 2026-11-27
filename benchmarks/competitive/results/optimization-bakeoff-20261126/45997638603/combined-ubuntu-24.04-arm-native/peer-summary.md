| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.45x | 1.69x | 1.84x | 2.19x | 2.21x | 30/30 |
| materialize | 1.01x | 1.98x | 1.54x | 1.35x | 2.19x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.03x | 1.64x | 1.51x | 2.09x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.83x | 1.51x | 2.22x | 2.10x | 30/30 |
| fen_parse | 1.01x | 1.24x | 1.34x | 1.26x | 1.76x | 1.42x | 30/30 |
| fen_write_native_ep | 1.00x | 4.46x | 9.17x | 1.57x | 29.91x | 2.45x | 30/30 |
| snapshot | 0.99x | 1.10x | 1.04x | 1.47x | 1.04x | 0.94x | 22/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.42x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.25x | 1.48x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.45x | 129.98x | 176.65x | 144.51x | 5/5 |
| chess960_materialize | 1.01x | 1.72x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.02x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.35x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.84x | 1.85x | 1.49x | 2.17x | 2.39x | 10/10 |
| corpus_materialize | 1.00x | 1.63x | 1.53x | 1.24x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.10x | 2.88x | 2.07x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.69x | 2.02x | 2.24x | 2.28x | 8/10 |
| replay_checked_native | 1.01x | 2.34x | 1.14x | 7.65x | 6.84x | 8.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
