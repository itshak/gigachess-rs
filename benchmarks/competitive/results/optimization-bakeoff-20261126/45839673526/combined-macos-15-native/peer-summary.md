| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.83x | 4.68x | 1.56x | 1.87x | 2.09x | 1.85x | 29/30 |
| materialize | 0.81x | 1.92x | 1.48x | 0.98x | 2.02x | 1.28x | 25/30 |
| perft_bulk_d4 | 0.90x | 3.88x | 1.42x | 1.28x | 2.03x | 1.89x | 29/30 |
| perft_full_d3 | 0.80x | 1.24x | 1.31x | 0.96x | 1.55x | 1.44x | 24/30 |
| fen_parse | 0.86x | 1.20x | 1.22x | 1.29x | 1.53x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 4.30x | 9.10x | 1.84x | 29.73x | 2.36x | 30/30 |
| snapshot | 0.96x | 1.49x | 1.10x | 2.04x | 1.01x | 0.83x | 20/30 |
| san_parse_all | 0.93x | 1.79x | N/A | 3.81x | N/A | N/A | 12/12 |
| san_render_native | 0.90x | 11.41x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 0.94x | 1.16x | 1.26x | 6.06x | 1.19x | 1.43x | 29/30 |
| make_unmake_full | 0.85x | N/A | N/A | 1.19x | N/A | N/A | 5/6 |
| startpos | 0.88x | 1.14x | 66.51x | 125.19x | 160.58x | 129.59x | 5/5 |
| chess960_materialize | 0.95x | 1.82x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 3.08x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.80x | 34.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.84x | 0.93x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.87x | 18.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.79x | 5.50x | 1.77x | 1.15x | 2.19x | 2.46x | 10/10 |
| corpus_materialize | 0.82x | 1.50x | 1.33x | 1.16x | 1.87x | 1.19x | 9/10 |
| corpus_uci_sorted | 0.85x | 1.79x | 2.55x | 1.87x | 2.39x | 1.26x | 10/10 |
| replay_trusted_native | 0.85x | 0.62x | 1.60x | 1.63x | 1.78x | 1.77x | 8/10 |
| replay_checked_native | 0.89x | 2.55x | 1.01x | 7.52x | 7.30x | 8.61x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
