| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 3.02x | 1.49x | 1.35x | 1.87x | 1.33x | 30/30 |
| materialize | 0.96x | 2.02x | 1.80x | 1.33x | 2.35x | 1.58x | 30/30 |
| perft_bulk_d4 | 0.97x | 2.79x | 1.51x | 1.17x | 1.90x | 1.63x | 30/30 |
| perft_full_d3 | 0.95x | 1.43x | 1.64x | 1.32x | 2.18x | 1.78x | 30/30 |
| fen_parse | 1.05x | 1.53x | 1.63x | 1.34x | 2.41x | 1.42x | 30/30 |
| fen_write_native_ep | 0.96x | 2.79x | 9.09x | 1.61x | 25.50x | 1.95x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.22x | 3.05x | 1.01x | 1.07x | 30/30 |
| san_parse_all | 1.03x | 2.08x | N/A | 4.51x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.96x | N/A | 2.41x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.91x | 1.33x | 3.97x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.34x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.28x | 65.41x | 108.91x | 180.87x | 113.00x | 5/5 |
| chess960_materialize | 1.02x | 1.58x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.04x | 2.16x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 23.25x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 18.79x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 3.58x | 1.81x | 1.20x | 1.96x | 2.37x | 10/10 |
| corpus_materialize | 0.94x | 1.52x | 1.52x | 1.15x | 1.95x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.18x | 3.14x | 2.40x | 3.00x | 1.80x | 10/10 |
| replay_trusted_native | 1.01x | 0.69x | 1.36x | 1.60x | 2.13x | 1.99x | 8/10 |
| replay_checked_native | 1.01x | 2.18x | 1.06x | 7.57x | 7.64x | 8.61x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
