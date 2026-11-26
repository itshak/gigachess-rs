| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.92x | 1.49x | 1.61x | 1.94x | 1.93x | 30/30 |
| materialize | 1.00x | 1.85x | 1.51x | 1.27x | 2.04x | 1.41x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.42x | 1.41x | 1.27x | 1.79x | 1.84x | 30/30 |
| perft_full_d3 | 1.00x | 1.16x | 1.26x | 1.04x | 1.54x | 1.46x | 29/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.25x | 1.73x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.52x | 9.31x | 1.57x | 29.98x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.05x | 1.06x | 1.45x | 1.09x | 0.96x | 23/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.23x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.78x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.24x | 1.47x | 4.04x | 1.19x | 1.19x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.80x | 129.14x | 176.60x | 143.33x | 5/5 |
| chess960_materialize | 1.01x | 1.60x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.64x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 3.87x | 1.50x | 1.20x | 1.76x | 1.92x | 10/10 |
| corpus_materialize | 0.99x | 1.53x | 1.45x | 1.16x | 1.82x | 1.31x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.89x | 2.08x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.60x | 1.60x | 1.94x | 2.17x | 2.21x | 8/10 |
| replay_checked_native | 0.99x | 2.13x | 1.04x | 6.97x | 6.25x | 8.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
