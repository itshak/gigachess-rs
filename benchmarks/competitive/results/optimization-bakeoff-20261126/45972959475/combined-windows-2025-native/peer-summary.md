| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.92x | 1.73x | 1.66x | 2.48x | 2.14x | 30/30 |
| materialize | 1.02x | 2.06x | 1.38x | 1.29x | 2.37x | 1.59x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.10x | 1.64x | 1.37x | 2.33x | 2.09x | 30/30 |
| perft_full_d3 | 1.07x | 1.45x | 1.52x | 1.42x | 2.01x | 1.94x | 30/30 |
| fen_parse | 1.10x | 1.37x | 1.42x | 1.37x | 2.29x | 1.51x | 30/30 |
| fen_write_native_ep | 0.99x | 4.21x | 8.43x | 1.73x | 33.25x | 2.16x | 30/30 |
| snapshot | 1.04x | 1.89x | 1.00x | 4.27x | 0.99x | 1.01x | 19/30 |
| san_parse_all | 1.03x | 2.06x | N/A | 5.57x | N/A | N/A | 12/12 |
| san_render_native | 1.06x | 11.28x | N/A | 3.88x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.37x | 1.44x | 11.46x | 1.51x | 1.71x | 30/30 |
| make_unmake_full | 1.09x | N/A | N/A | 1.42x | N/A | N/A | 6/6 |
| startpos | 1.44x | 1.25x | 79.53x | 155.42x | 235.25x | 173.75x | 5/5 |
| chess960_materialize | 1.12x | 1.65x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.94x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 25.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.88x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.42x | 1.86x | 1.37x | 2.52x | 2.44x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.26x | 1.18x | 2.02x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.78x | 2.19x | 1.82x | 2.16x | 1.42x | 10/10 |
| replay_trusted_native | 1.02x | 0.83x | 1.58x | 2.05x | 2.59x | 3.22x | 9/10 |
| replay_checked_native | 1.01x | 2.47x | 1.08x | 8.56x | 8.31x | 9.71x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
