| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.48x | 1.55x | 1.57x | 2.38x | 1.79x | 30/30 |
| materialize | 0.97x | 1.86x | 1.53x | 1.23x | 2.14x | 1.47x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.81x | 1.46x | 1.32x | 2.30x | 1.79x | 30/30 |
| perft_full_d3 | 1.02x | 1.28x | 1.43x | 1.25x | 1.84x | 1.71x | 30/30 |
| fen_parse | 1.14x | 1.35x | 1.40x | 1.27x | 2.10x | 1.38x | 30/30 |
| fen_write_native_ep | 1.05x | 3.98x | 9.90x | 1.68x | 27.85x | 1.98x | 30/30 |
| snapshot | 0.95x | 1.43x | 0.90x | 2.35x | 0.90x | 0.90x | 12/30 |
| san_parse_all | 0.98x | 2.06x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.56x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.23x | 4.13x | 1.33x | 1.48x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.08x | 1.25x | 77.95x | 150.78x | 221.43x | 154.03x | 5/5 |
| chess960_materialize | 1.08x | 1.55x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.75x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 27.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.88x | 18.57x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 5.26x | 1.71x | 1.32x | 2.54x | 2.15x | 10/10 |
| corpus_materialize | 0.95x | 1.53x | 1.49x | 1.13x | 1.93x | 1.29x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.37x | 3.13x | 2.38x | 3.05x | 1.69x | 10/10 |
| replay_trusted_native | 1.04x | 0.79x | 1.50x | 1.74x | 2.29x | 2.53x | 8/10 |
| replay_checked_native | 1.04x | 2.48x | 1.06x | 8.18x | 8.05x | 9.33x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
