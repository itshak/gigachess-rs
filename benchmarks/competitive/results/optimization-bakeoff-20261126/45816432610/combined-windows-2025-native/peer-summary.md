| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.55x | 1.57x | 1.49x | 2.21x | 1.92x | 30/30 |
| materialize | 0.99x | 1.94x | 1.27x | 1.10x | 2.23x | 1.40x | 29/30 |
| perft_bulk_d4 | 1.02x | 3.94x | 1.43x | 1.23x | 2.23x | 2.08x | 30/30 |
| perft_full_d3 | 1.24x | 1.37x | 1.47x | 1.24x | 1.98x | 2.01x | 30/30 |
| fen_parse | 0.99x | 1.41x | 1.60x | 1.57x | 2.23x | 1.66x | 30/30 |
| fen_write_native_ep | 1.01x | 3.83x | 7.47x | 1.52x | 31.93x | 2.04x | 30/30 |
| snapshot | 1.22x | 2.51x | 1.50x | 5.46x | 1.51x | 1.50x | 30/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.69x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 10.45x | N/A | 3.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.17x | 1.65x | 1.21x | 8.00x | 1.25x | 1.68x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.26x | 90.87x | 171.37x | 239.29x | 184.60x | 5/5 |
| chess960_materialize | 1.18x | 1.61x | 1.30x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.78x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.10x | 37.59x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.63x | 23.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 5.43x | 1.64x | 1.09x | 2.19x | 2.05x | 9/10 |
| corpus_materialize | 0.93x | 1.61x | 1.24x | 1.04x | 2.19x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.09x | 1.76x | 2.12x | 1.88x | 2.10x | 1.49x | 10/10 |
| replay_trusted_native | 0.93x | 0.59x | 1.20x | 1.39x | 2.12x | 2.49x | 8/10 |
| replay_checked_native | 1.04x | 2.12x | 0.98x | 7.34x | 7.62x | 9.21x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
