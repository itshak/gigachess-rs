| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.89x | 1.75x | 1.78x | 2.72x | 2.03x | 30/30 |
| materialize | 1.03x | 2.09x | 1.75x | 1.20x | 2.39x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.16x | 1.64x | 1.52x | 2.58x | 2.00x | 30/30 |
| perft_full_d3 | 1.01x | 1.27x | 1.60x | 1.28x | 2.00x | 1.83x | 30/30 |
| fen_parse | 0.88x | 1.25x | 1.28x | 1.08x | 2.04x | 1.17x | 30/30 |
| fen_write_native_ep | 1.00x | 4.08x | 9.76x | 1.93x | 27.94x | 2.31x | 30/30 |
| snapshot | 1.00x | 1.88x | 1.25x | 3.25x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.02x | N/A | 3.87x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.56x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.11x | 1.33x | 5.49x | 1.43x | 1.62x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 95.76x | 146.59x | 234.23x | 151.30x | 5/5 |
| chess960_materialize | 1.06x | 1.75x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.77x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 19.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.84x | 1.90x | 1.47x | 2.64x | 2.19x | 10/10 |
| corpus_materialize | 1.02x | 1.85x | 1.74x | 1.26x | 3.01x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.39x | 3.20x | 2.43x | 3.02x | 1.74x | 10/10 |
| replay_trusted_native | 1.01x | 1.11x | 2.10x | 2.35x | 3.49x | 4.01x | 10/10 |
| replay_checked_native | 1.00x | 2.56x | 1.14x | 6.26x | 7.84x | 7.99x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
