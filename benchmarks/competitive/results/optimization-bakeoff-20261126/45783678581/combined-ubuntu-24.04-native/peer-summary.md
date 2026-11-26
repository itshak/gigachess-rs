| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.03x | 1.72x | 1.46x | 2.25x | 1.76x | 30/30 |
| materialize | 0.95x | 2.01x | 1.50x | 1.65x | 2.33x | 1.81x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.94x | 1.60x | 1.21x | 2.40x | 2.02x | 30/30 |
| perft_full_d3 | 0.99x | 1.56x | 1.72x | 1.67x | 2.25x | 1.84x | 30/30 |
| fen_parse | 1.15x | 1.41x | 1.64x | 1.12x | 1.63x | 1.19x | 30/30 |
| fen_write_native_ep | 0.98x | 3.19x | 6.59x | 1.34x | 18.19x | 1.69x | 30/30 |
| snapshot | 1.41x | 1.83x | 1.15x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 1.88x | N/A | 4.39x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.10x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.11x | 1.97x | 1.22x | 3.61x | 1.25x | 1.47x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.22x | 1.12x | 88.57x | 153.40x | 191.06x | 156.12x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.27x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.51x | 2.43x | 1.35x | 2.99x | 2.69x | 10/10 |
| corpus_materialize | 1.02x | 1.45x | 1.33x | 1.10x | 1.78x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 3.00x | 2.23x | 2.94x | 1.58x | 10/10 |
| replay_trusted_native | 1.07x | 0.78x | 1.41x | 1.35x | 1.94x | 2.23x | 7/10 |
| replay_checked_native | 1.00x | 2.02x | 1.12x | 8.45x | 8.23x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
