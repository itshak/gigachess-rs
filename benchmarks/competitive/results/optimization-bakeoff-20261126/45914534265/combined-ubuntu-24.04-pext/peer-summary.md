| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.52x | 1.88x | 1.63x | 2.66x | 1.93x | 30/30 |
| materialize | 0.98x | 2.13x | 1.56x | 1.72x | 2.63x | 1.98x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.87x | 1.53x | 1.20x | 2.37x | 1.89x | 29/30 |
| perft_full_d3 | 0.99x | 1.39x | 1.50x | 1.51x | 1.96x | 1.64x | 30/30 |
| fen_parse | 1.01x | 1.36x | 1.41x | 1.07x | 1.62x | 1.15x | 30/30 |
| fen_write_native_ep | 1.00x | 4.43x | 10.24x | 2.13x | 30.22x | 2.65x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.02x | 2.05x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 7.85x | N/A | 1.79x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.03x | 1.34x | 2.35x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 112.22x | 154.56x | 209.73x | 162.56x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 39.75x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 19.41x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.46x | 2.02x | 1.13x | 2.46x | 2.26x | 10/10 |
| corpus_materialize | 1.02x | 1.44x | 1.30x | 1.09x | 1.77x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 3.03x | 2.22x | 2.95x | 1.55x | 10/10 |
| replay_trusted_native | 1.02x | 0.93x | 1.59x | 1.62x | 2.31x | 2.62x | 9/10 |
| replay_checked_native | 0.99x | 2.15x | 1.12x | 9.06x | 8.83x | 10.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
