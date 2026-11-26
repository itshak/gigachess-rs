| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.93x | 4.79x | 1.59x | 1.47x | 2.35x | 1.93x | 30/30 |
| materialize | 0.93x | 2.04x | 1.34x | 1.13x | 2.27x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.85x | 1.39x | 1.18x | 2.16x | 1.98x | 30/30 |
| perft_full_d3 | 0.93x | 1.32x | 1.37x | 1.17x | 1.84x | 2.33x | 30/30 |
| fen_parse | 0.95x | 1.30x | 1.47x | 1.52x | 2.09x | 1.49x | 30/30 |
| fen_write_native_ep | 0.92x | 3.50x | 6.61x | 1.39x | 28.71x | 1.89x | 30/30 |
| snapshot | 0.92x | 2.30x | 1.38x | 4.16x | 1.40x | 1.40x | 30/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.70x | N/A | N/A | 12/12 |
| san_render_native | 0.78x | 8.83x | N/A | 3.18x | N/A | N/A | 12/12 |
| apply_children_full | 0.61x | 1.34x | 1.04x | 5.51x | 0.86x | 1.18x | 23/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.07x | 0.91x | 63.47x | 129.04x | 172.99x | 129.25x | 4/5 |
| chess960_materialize | 0.85x | 1.47x | 1.20x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.65x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 34.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 26.91x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.44x | 1.63x | 1.09x | 2.14x | 2.05x | 9/10 |
| corpus_materialize | 1.04x | 1.64x | 1.28x | 1.10x | 2.22x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.11x | 1.75x | 2.12x | 1.90x | 2.06x | 1.49x | 10/10 |
| replay_trusted_native | 1.12x | 0.80x | 1.59x | 1.94x | 2.98x | 3.74x | 9/10 |
| replay_checked_native | 0.95x | 2.39x | 1.12x | 8.30x | 8.62x | 10.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
