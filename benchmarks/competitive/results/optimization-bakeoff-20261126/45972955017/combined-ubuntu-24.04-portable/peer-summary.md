| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.19x | 1.64x | 1.77x | 2.06x | 1.44x | 30/30 |
| materialize | 1.00x | 2.14x | 1.95x | 1.39x | 2.58x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.11x | 1.69x | 1.31x | 2.11x | 1.81x | 30/30 |
| perft_full_d3 | 1.01x | 1.39x | 1.60x | 1.27x | 1.90x | 1.72x | 30/30 |
| fen_parse | 0.86x | 1.44x | 1.35x | 1.06x | 2.14x | 1.18x | 30/30 |
| fen_write_native_ep | 1.16x | 2.77x | 9.64x | 1.66x | 26.86x | 1.99x | 30/30 |
| snapshot | 1.00x | 2.06x | 1.65x | 2.90x | 1.02x | 0.98x | 24/30 |
| san_parse_all | 1.01x | 2.04x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.76x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.28x | 1.61x | 4.66x | 1.53x | 1.52x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 66.23x | 109.07x | 192.50x | 118.76x | 5/5 |
| chess960_materialize | 1.01x | 1.68x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.31x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 25.95x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.88x | 1.93x | 1.29x | 2.12x | 2.57x | 10/10 |
| corpus_materialize | 1.01x | 1.63x | 1.63x | 1.22x | 2.10x | 1.38x | 10/10 |
| corpus_uci_sorted | 0.94x | 2.02x | 3.14x | 2.26x | 2.93x | 1.58x | 10/10 |
| replay_trusted_native | 1.00x | 0.93x | 1.80x | 1.98x | 2.71x | 2.47x | 9/10 |
| replay_checked_native | 0.98x | 2.30x | 1.14x | 7.91x | 8.15x | 9.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
