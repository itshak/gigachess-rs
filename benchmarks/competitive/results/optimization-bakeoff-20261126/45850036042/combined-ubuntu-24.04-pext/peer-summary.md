| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.96x | 1.68x | 1.71x | 2.60x | 2.00x | 30/30 |
| materialize | 1.00x | 1.98x | 1.69x | 1.26x | 2.34x | 1.49x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.06x | 1.54x | 1.42x | 2.43x | 1.89x | 30/30 |
| perft_full_d3 | 1.01x | 1.37x | 1.52x | 1.36x | 1.99x | 1.84x | 30/30 |
| fen_parse | 1.02x | 1.42x | 1.38x | 1.22x | 2.21x | 1.28x | 30/30 |
| fen_write_native_ep | 1.01x | 4.05x | 9.65x | 1.75x | 28.06x | 2.23x | 30/30 |
| snapshot | 1.00x | 1.97x | 1.25x | 3.56x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.00x | 2.10x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.19x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.25x | 1.29x | 4.49x | 1.44x | 1.87x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.72x | 146.10x | 229.92x | 151.68x | 5/5 |
| chess960_materialize | 0.99x | 1.60x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.83x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.86x | 24.27x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.80x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.72x | 1.85x | 1.45x | 2.73x | 2.36x | 10/10 |
| corpus_materialize | 0.98x | 1.62x | 1.56x | 1.18x | 2.04x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.43x | 3.18x | 2.44x | 3.05x | 1.74x | 10/10 |
| replay_trusted_native | 1.01x | 0.86x | 1.60x | 1.89x | 2.51x | 2.81x | 9/10 |
| replay_checked_native | 1.01x | 2.32x | 1.02x | 8.08x | 7.77x | 8.88x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
