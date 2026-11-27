| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.47x | 1.71x | 1.83x | 2.22x | 2.20x | 30/30 |
| materialize | 1.01x | 1.96x | 1.55x | 1.35x | 2.17x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.03x | 1.65x | 1.50x | 2.10x | 2.16x | 30/30 |
| perft_full_d3 | 1.14x | 1.68x | 1.81x | 1.52x | 2.23x | 2.10x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.32x | 1.25x | 1.72x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 4.53x | 9.22x | 1.56x | 30.10x | 2.38x | 30/30 |
| snapshot | 1.01x | 1.14x | 1.01x | 1.49x | 1.00x | 0.94x | 18/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.27x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 1.19x | 1.40x | 3.84x | 1.15x | 1.14x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.49x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.49x | 130.34x | 176.48x | 142.57x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.02x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.69x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.90x | 1.88x | 1.54x | 2.22x | 2.43x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.53x | 1.24x | 1.92x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.94x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 0.90x | 0.57x | 1.51x | 1.83x | 2.03x | 2.09x | 8/10 |
| replay_checked_native | 0.96x | 2.24x | 1.07x | 7.28x | 6.48x | 8.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
