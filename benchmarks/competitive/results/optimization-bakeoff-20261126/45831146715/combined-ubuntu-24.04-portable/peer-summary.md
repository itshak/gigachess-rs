| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.70x | 1.56x | 1.35x | 1.87x | 1.33x | 29/30 |
| materialize | 1.00x | 2.26x | 1.82x | 1.77x | 2.77x | 2.05x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.71x | 1.43x | 1.06x | 1.83x | 1.64x | 29/30 |
| perft_full_d3 | 0.88x | 1.31x | 1.53x | 1.34x | 1.82x | 1.53x | 30/30 |
| fen_parse | 0.99x | 1.43x | 1.34x | 1.10x | 1.38x | 1.13x | 30/30 |
| fen_write_native_ep | 0.98x | 2.99x | 9.68x | 1.75x | 26.69x | 2.59x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 24/30 |
| san_parse_all | 1.00x | 2.10x | N/A | 4.47x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.99x | N/A | 2.04x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 1.87x | 1.23x | 3.55x | 1.14x | 1.33x | 30/30 |
| make_unmake_full | 0.83x | N/A | N/A | 1.04x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.36x | 60.26x | 110.84x | 128.42x | 111.83x | 5/5 |
| chess960_materialize | 0.92x | 1.45x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 1.85x | 1.31x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 40.68x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 0.97x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.06x | 20.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 3.63x | 1.81x | 1.01x | 1.85x | 2.54x | 9/10 |
| corpus_materialize | 1.00x | 1.44x | 1.37x | 1.09x | 1.75x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.96x | 2.93x | 2.21x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 0.79x | 0.72x | 1.31x | 1.28x | 1.77x | 1.66x | 7/10 |
| replay_checked_native | 0.98x | 1.97x | 1.01x | 8.08x | 7.85x | 8.89x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
