| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.66x | 1.60x | 1.37x | 2.13x | 1.73x | 30/30 |
| materialize | 1.05x | 1.98x | 1.51x | 1.58x | 2.28x | 1.72x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.21x | 1.31x | 1.00x | 1.96x | 1.59x | 26/30 |
| perft_full_d3 | 1.01x | 1.51x | 1.84x | 1.64x | 2.16x | 1.88x | 30/30 |
| fen_parse | 1.09x | 1.29x | 1.46x | 1.15x | 1.48x | 1.22x | 30/30 |
| fen_write_native_ep | 0.95x | 4.66x | 9.21x | 1.80x | 27.37x | 2.49x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.34x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.98x | 1.84x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.06x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.12x | 1.36x | 4.17x | 1.33x | 1.69x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 90.98x | 154.70x | 195.72x | 158.75x | 5/5 |
| chess960_materialize | 0.99x | 1.51x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.26x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.62x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 24.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.07x | 1.59x | 0.82x | 1.91x | 1.79x | 8/10 |
| corpus_materialize | 1.00x | 1.35x | 1.24x | 1.01x | 1.63x | 1.13x | 9/10 |
| corpus_uci_sorted | 1.02x | 2.24x | 2.90x | 2.23x | 2.83x | 1.54x | 10/10 |
| replay_trusted_native | 1.03x | 0.91x | 1.65x | 1.60x | 2.23x | 2.58x | 9/10 |
| replay_checked_native | 1.02x | 2.06x | 1.09x | 8.84x | 8.57x | 10.14x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
