| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.68x | 1.79x | 2.17x | 2.16x | 30/30 |
| materialize | 1.00x | 1.95x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.59x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.47x | 1.59x | 1.33x | 1.93x | 1.85x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.30x | 1.24x | 1.74x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.33x | 1.58x | 29.91x | 2.39x | 30/30 |
| snapshot | 1.08x | 1.21x | 1.05x | 1.60x | 1.03x | 1.02x | 29/30 |
| san_parse_all | 1.01x | 1.84x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.46x | 4.05x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 0.90x | 1.13x | 68.93x | 116.96x | 156.85x | 126.74x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.94x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.40x | 1.73x | 1.37x | 2.00x | 2.19x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.54x | 1.22x | 1.91x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.82x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.64x | 2.02x | 2.22x | 2.27x | 8/10 |
| replay_checked_native | 1.00x | 2.14x | 1.04x | 7.06x | 6.29x | 8.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
