| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.99x | 1.74x | 1.72x | 2.42x | 2.16x | 30/30 |
| materialize | 0.99x | 1.97x | 1.33x | 1.30x | 2.25x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.07x | 1.68x | 1.34x | 2.28x | 2.13x | 30/30 |
| perft_full_d3 | 0.97x | 1.26x | 1.32x | 1.07x | 1.56x | 1.50x | 28/30 |
| fen_parse | 1.07x | 1.22x | 1.39x | 1.21x | 2.09x | 1.34x | 30/30 |
| fen_write_native_ep | 0.94x | 4.02x | 7.79x | 1.59x | 29.50x | 1.85x | 30/30 |
| snapshot | 0.96x | 1.93x | 1.01x | 4.22x | 1.05x | 1.10x | 26/30 |
| san_parse_all | 0.98x | 2.14x | N/A | 5.79x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.44x | N/A | 3.89x | N/A | N/A | 12/12 |
| apply_children_full | 0.91x | 2.05x | 1.26x | 8.45x | 1.33x | 1.49x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.25x | 79.56x | 151.85x | 241.30x | 176.37x | 5/5 |
| chess960_materialize | 1.07x | 1.65x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.54x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.62x | 0.61x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.98x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.51x | 1.78x | 1.40x | 2.56x | 2.47x | 10/10 |
| corpus_materialize | 1.02x | 1.59x | 1.25x | 1.20x | 2.00x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.74x | 2.15x | 1.82x | 2.13x | 1.40x | 10/10 |
| replay_trusted_native | 1.02x | 0.85x | 1.61x | 2.09x | 2.64x | 3.01x | 9/10 |
| replay_checked_native | 0.95x | 2.19x | 0.98x | 8.03x | 7.35x | 8.75x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
