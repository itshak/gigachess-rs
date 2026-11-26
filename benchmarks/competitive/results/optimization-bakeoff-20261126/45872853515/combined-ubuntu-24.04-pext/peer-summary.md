| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.47x | 1.87x | 1.63x | 2.62x | 1.91x | 30/30 |
| materialize | 1.00x | 2.26x | 1.69x | 1.76x | 2.76x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.85x | 1.53x | 1.21x | 2.44x | 1.87x | 29/30 |
| perft_full_d3 | 1.23x | 1.66x | 1.87x | 1.85x | 2.40x | 1.96x | 30/30 |
| fen_parse | 0.94x | 1.18x | 1.31x | 1.09x | 1.49x | 1.18x | 30/30 |
| fen_write_native_ep | 1.02x | 4.95x | 10.60x | 2.08x | 31.45x | 2.22x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 2.05x | N/A | 4.37x | N/A | N/A | 12/12 |
| san_render_native | 1.21x | 9.41x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.33x | 4.06x | 1.28x | 2.29x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.24x | 103.09x | 184.29x | 229.05x | 195.18x | 5/5 |
| chess960_materialize | 0.94x | 1.65x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 38.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 22.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.12x | 1.90x | 1.05x | 2.30x | 2.13x | 9/10 |
| corpus_materialize | 1.01x | 1.43x | 1.32x | 1.09x | 1.72x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.30x | 3.14x | 2.26x | 3.05x | 1.58x | 10/10 |
| replay_trusted_native | 0.96x | 0.85x | 1.49x | 1.59x | 2.20x | 2.77x | 9/10 |
| replay_checked_native | 1.01x | 2.19x | 1.13x | 9.30x | 8.96x | 10.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
