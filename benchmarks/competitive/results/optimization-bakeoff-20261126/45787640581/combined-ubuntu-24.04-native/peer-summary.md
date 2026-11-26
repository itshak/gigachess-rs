| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 3.87x | 1.68x | 1.40x | 2.30x | 1.72x | 30/30 |
| materialize | 0.99x | 2.06x | 1.57x | 1.62x | 2.48x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.85x | 3.28x | 1.35x | 1.03x | 2.09x | 1.62x | 28/30 |
| perft_full_d3 | 0.96x | 1.47x | 1.73x | 1.63x | 2.09x | 1.78x | 30/30 |
| fen_parse | 0.95x | 1.18x | 1.23x | 1.09x | 1.38x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 4.89x | 10.01x | 1.88x | 29.44x | 2.17x | 30/30 |
| snapshot | 0.77x | 1.42x | 1.15x | 5.36x | 1.15x | 1.15x | 30/30 |
| san_parse_all | 0.97x | 2.05x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.10x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.74x | 1.12x | 3.26x | 1.10x | 1.38x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.21x | 81.89x | 153.37x | 183.10x | 159.90x | 5/5 |
| chess960_materialize | 0.96x | 1.61x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.32x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 40.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.94x | 19.84x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.69x | 4.43x | 1.67x | 0.92x | 2.05x | 1.86x | 8/10 |
| corpus_materialize | 0.95x | 1.35x | 1.26x | 1.04x | 1.65x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.26x | 3.06x | 2.20x | 2.96x | 1.54x | 10/10 |
| replay_trusted_native | 1.01x | 0.70x | 1.31x | 1.31x | 1.82x | 2.14x | 8/10 |
| replay_checked_native | 1.00x | 2.03x | 1.12x | 8.88x | 8.28x | 9.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
