| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.70x | 1.55x | 1.35x | 1.81x | 1.33x | 29/30 |
| materialize | 1.00x | 2.13x | 1.74x | 1.74x | 2.63x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.74x | 1.45x | 1.07x | 1.79x | 1.64x | 29/30 |
| perft_full_d3 | 0.99x | 1.50x | 1.72x | 1.56x | 2.05x | 1.73x | 30/30 |
| fen_parse | 0.98x | 1.57x | 1.44x | 1.22x | 1.59x | 1.22x | 30/30 |
| fen_write_native_ep | 0.99x | 3.07x | 9.93x | 1.82x | 26.43x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.08x | 1.90x | N/A | 4.16x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.27x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.98x | 1.31x | 3.83x | 1.22x | 1.41x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 57.18x | 111.78x | 127.97x | 103.16x | 5/5 |
| chess960_materialize | 1.01x | 1.60x | 1.62x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.97x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 43.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 17.46x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.79x | 1.91x | 1.04x | 2.02x | 2.64x | 10/10 |
| corpus_materialize | 1.00x | 1.44x | 1.34x | 1.08x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.92x | 2.89x | 2.17x | 2.80x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.54x | 1.51x | 2.09x | 1.92x | 8/10 |
| replay_checked_native | 0.99x | 2.06x | 1.14x | 8.62x | 8.55x | 9.55x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
