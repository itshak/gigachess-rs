| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.05x | 3.39x | 1.74x | 1.55x | 2.20x | 1.53x | 30/30 |
| materialize | 1.02x | 2.15x | 1.93x | 1.38x | 2.59x | 1.65x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.26x | 1.78x | 1.37x | 2.22x | 1.91x | 30/30 |
| perft_full_d3 | 1.18x | 1.61x | 1.87x | 1.51x | 2.26x | 2.11x | 30/30 |
| fen_parse | 1.01x | 1.52x | 1.53x | 1.31x | 2.24x | 1.30x | 30/30 |
| fen_write_native_ep | 1.00x | 2.89x | 9.66x | 1.68x | 26.73x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.70x | 1.23x | 3.04x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.29x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.35x | 1.60x | 4.69x | 1.48x | 1.42x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 64.88x | 118.08x | 183.43x | 114.39x | 5/5 |
| chess960_materialize | 0.98x | 1.64x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.33x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.22x | 1.25x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.07x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 4.19x | 2.16x | 1.39x | 2.32x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.62x | 1.22x | 2.14x | 1.39x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.17x | 3.18x | 2.44x | 3.04x | 1.70x | 10/10 |
| replay_trusted_native | 0.94x | 0.86x | 1.61x | 1.89x | 2.49x | 2.38x | 9/10 |
| replay_checked_native | 0.99x | 2.26x | 1.15x | 8.06x | 7.97x | 9.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
