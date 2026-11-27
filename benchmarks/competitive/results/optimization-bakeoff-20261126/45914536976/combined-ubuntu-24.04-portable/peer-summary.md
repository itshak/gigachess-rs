| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.19x | 1.62x | 1.50x | 2.07x | 1.46x | 30/30 |
| materialize | 1.00x | 2.12x | 1.95x | 1.34x | 2.59x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.13x | 1.68x | 1.30x | 2.12x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.39x | 1.58x | 1.29x | 1.90x | 1.73x | 30/30 |
| fen_parse | 0.96x | 1.59x | 1.56x | 1.20x | 2.26x | 1.32x | 30/30 |
| fen_write_native_ep | 1.05x | 2.89x | 9.74x | 1.74x | 26.64x | 2.01x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.09x | 3.04x | 1.01x | 1.06x | 29/30 |
| san_parse_all | 1.00x | 1.99x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.06x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.27x | 1.59x | 4.62x | 1.46x | 1.50x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 65.01x | 106.47x | 178.52x | 110.64x | 5/5 |
| chess960_materialize | 0.98x | 1.63x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.27x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 22.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 18.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 3.91x | 1.94x | 1.31x | 2.15x | 2.62x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.61x | 1.22x | 2.08x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.21x | 3.22x | 2.44x | 3.09x | 1.74x | 10/10 |
| replay_trusted_native | 1.00x | 0.92x | 1.71x | 2.01x | 2.65x | 2.47x | 9/10 |
| replay_checked_native | 0.95x | 2.21x | 1.10x | 7.93x | 7.90x | 8.73x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
