| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.50x | 1.88x | 1.63x | 2.56x | 1.93x | 30/30 |
| materialize | 1.05x | 2.35x | 1.75x | 1.78x | 2.74x | 2.01x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.52x | 1.18x | 2.36x | 1.87x | 29/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.57x | 1.51x | 1.99x | 1.66x | 30/30 |
| fen_parse | 1.00x | 1.35x | 1.57x | 1.07x | 1.68x | 1.24x | 30/30 |
| fen_write_native_ep | 0.99x | 4.54x | 9.95x | 2.11x | 28.60x | 2.63x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.66x | 1.00x | 1.33x | 22/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.32x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 8.94x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.35x | 4.05x | 1.44x | 1.65x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 80.22x | 138.91x | 196.54x | 151.93x | 5/5 |
| chess960_materialize | 1.00x | 1.84x | 1.73x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.51x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 39.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 23.89x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.48x | 2.04x | 1.12x | 2.49x | 2.27x | 10/10 |
| corpus_materialize | 1.01x | 1.42x | 1.29x | 1.08x | 1.73x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.25x | 2.90x | 2.20x | 2.83x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.91x | 1.66x | 1.58x | 2.30x | 2.60x | 9/10 |
| replay_checked_native | 1.00x | 2.16x | 1.17x | 9.18x | 8.83x | 10.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
