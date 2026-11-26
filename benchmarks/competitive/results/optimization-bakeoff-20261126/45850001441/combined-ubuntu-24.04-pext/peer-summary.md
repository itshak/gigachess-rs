| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.49x | 1.88x | 1.64x | 2.52x | 1.91x | 30/30 |
| materialize | 1.00x | 2.26x | 1.75x | 1.80x | 2.85x | 1.91x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.83x | 1.52x | 1.19x | 2.35x | 1.86x | 29/30 |
| perft_full_d3 | 1.22x | 1.65x | 1.84x | 1.84x | 2.40x | 2.59x | 30/30 |
| fen_parse | 0.93x | 1.29x | 1.29x | 1.07x | 1.48x | 1.06x | 29/30 |
| fen_write_native_ep | 1.00x | 4.68x | 10.39x | 2.17x | 30.33x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.11x | 1.94x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.00x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.08x | 1.31x | 3.94x | 1.31x | 1.64x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 88.05x | 161.10x | 204.31x | 159.11x | 5/5 |
| chess960_materialize | 0.96x | 1.70x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.50x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 19.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 5.47x | 2.04x | 1.13x | 2.49x | 2.25x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.30x | 1.08x | 1.76x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.36x | 3.07x | 2.25x | 2.98x | 1.56x | 10/10 |
| replay_trusted_native | 0.97x | 0.89x | 1.51x | 1.56x | 2.19x | 2.49x | 9/10 |
| replay_checked_native | 1.02x | 2.03x | 1.05x | 8.48x | 8.29x | 9.61x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
