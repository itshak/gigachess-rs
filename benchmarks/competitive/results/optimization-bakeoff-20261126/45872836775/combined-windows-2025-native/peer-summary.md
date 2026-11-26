| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.83x | 1.78x | 1.72x | 2.64x | 2.18x | 30/30 |
| materialize | 1.03x | 2.02x | 1.34x | 1.15x | 2.33x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.07x | 1.64x | 1.44x | 2.46x | 2.25x | 30/30 |
| perft_full_d3 | 1.00x | 1.49x | 1.56x | 1.29x | 2.11x | 1.87x | 30/30 |
| fen_parse | 1.06x | 1.46x | 1.66x | 1.50x | 2.27x | 1.58x | 30/30 |
| fen_write_native_ep | 1.02x | 4.00x | 7.94x | 1.59x | 31.86x | 2.01x | 30/30 |
| snapshot | 0.97x | 1.77x | 1.22x | 4.02x | 1.22x | 1.08x | 30/30 |
| san_parse_all | 1.00x | 1.99x | N/A | 4.75x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.15x | N/A | 3.30x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.95x | 1.35x | 8.29x | 1.54x | 1.75x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.01x | 87.39x | 133.14x | 197.53x | 142.87x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.92x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 19.76x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.81x | 0.82x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 19.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.40x | 1.77x | 1.31x | 2.36x | 2.13x | 9/10 |
| corpus_materialize | 0.99x | 1.76x | 1.38x | 1.18x | 2.81x | 1.51x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.86x | 2.38x | 1.99x | 2.29x | 1.50x | 10/10 |
| replay_trusted_native | 0.98x | 1.07x | 1.87x | 2.31x | 3.63x | 4.52x | 9/10 |
| replay_checked_native | 1.00x | 2.49x | 1.10x | 6.65x | 7.87x | 8.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
