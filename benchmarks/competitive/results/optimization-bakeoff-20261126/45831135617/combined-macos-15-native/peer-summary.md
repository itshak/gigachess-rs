| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.88x | 4.62x | 1.42x | 1.70x | 2.12x | 1.87x | 28/30 |
| materialize | 0.85x | 2.07x | 1.40x | 1.05x | 2.26x | 1.37x | 27/30 |
| perft_bulk_d4 | 0.87x | 3.36x | 1.29x | 1.06x | 1.65x | 1.58x | 27/30 |
| perft_full_d3 | 0.86x | 1.47x | 1.67x | 1.27x | 1.84x | 1.93x | 30/30 |
| fen_parse | 0.97x | 1.31x | 1.36x | 1.42x | 1.72x | 1.45x | 30/30 |
| fen_write_native_ep | 1.01x | 4.61x | 9.29x | 1.89x | 30.31x | 2.25x | 30/30 |
| snapshot | 0.95x | 1.51x | 1.10x | 2.35x | 1.16x | 0.82x | 20/30 |
| san_parse_all | 0.98x | 1.71x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.90x | N/A | 2.91x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.11x | 1.26x | 6.12x | 1.22x | 1.42x | 27/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.19x | N/A | N/A | 5/6 |
| startpos | 2.91x | 2.67x | 129.77x | 274.00x | 312.32x | 280.92x | 5/5 |
| chess960_materialize | 0.77x | 1.57x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.79x | 3.02x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 41.05x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.85x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.90x | 20.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.90x | 4.38x | 1.50x | 0.94x | 1.82x | 1.82x | 8/10 |
| corpus_materialize | 1.12x | 1.53x | 1.31x | 1.04x | 1.85x | 1.20x | 9/10 |
| corpus_uci_sorted | 0.89x | 1.85x | 2.13x | 1.87x | 2.28x | 1.32x | 10/10 |
| replay_trusted_native | 1.07x | 0.64x | 1.53x | 1.66x | 2.01x | 1.82x | 8/10 |
| replay_checked_native | 0.95x | 2.16x | 1.06x | 7.43x | 6.96x | 7.96x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
