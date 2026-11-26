| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.96x | 1.47x | 1.39x | 1.93x | 1.37x | 30/30 |
| materialize | 1.02x | 2.01x | 1.83x | 1.30x | 2.44x | 1.54x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.79x | 1.49x | 1.17x | 1.90x | 1.64x | 30/30 |
| perft_full_d3 | 1.00x | 1.38x | 1.62x | 1.34x | 1.93x | 1.76x | 30/30 |
| fen_parse | 1.13x | 1.46x | 1.45x | 1.28x | 2.19x | 1.27x | 30/30 |
| fen_write_native_ep | 1.01x | 2.92x | 10.06x | 1.66x | 28.37x | 2.10x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.24x | 3.03x | 1.03x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 2.00x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.93x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.30x | 1.57x | 4.60x | 1.43x | 1.38x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.43x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.34x | 66.72x | 121.36x | 175.20x | 108.19x | 5/5 |
| chess960_materialize | 1.07x | 1.52x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.12x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 26.49x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.55x | 1.76x | 1.19x | 1.97x | 2.39x | 10/10 |
| corpus_materialize | 1.02x | 1.51x | 1.53x | 1.17x | 2.02x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.21x | 3.26x | 2.47x | 3.15x | 1.72x | 10/10 |
| replay_trusted_native | 1.03x | 0.93x | 1.80x | 2.01x | 2.61x | 2.45x | 9/10 |
| replay_checked_native | 1.02x | 2.15x | 1.10x | 7.59x | 7.53x | 8.43x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
