| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.08x | 4.52x | 1.91x | 1.64x | 2.30x | 1.98x | 30/30 |
| materialize | 1.00x | 2.03x | 1.60x | 1.64x | 2.32x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.98x | 1.61x | 1.23x | 2.24x | 1.93x | 29/30 |
| perft_full_d3 | 1.22x | 1.67x | 1.93x | 1.79x | 2.34x | 2.04x | 30/30 |
| fen_parse | 0.99x | 1.40x | 1.45x | 1.28x | 1.65x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 5.30x | 10.88x | 2.16x | 31.34x | 2.60x | 30/30 |
| snapshot | 0.99x | 1.82x | 1.15x | 4.29x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.03x | 1.82x | N/A | 4.26x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.29x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.14x | 1.36x | 4.22x | 1.35x | 1.71x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.14x | 91.86x | 167.26x | 213.14x | 179.36x | 5/5 |
| chess960_materialize | 0.99x | 1.56x | 1.43x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.82x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.57x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.62x | 1.68x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.04x | 26.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.40x | 6.51x | 2.56x | 1.29x | 2.91x | 2.79x | 10/10 |
| corpus_materialize | 1.01x | 1.37x | 1.29x | 1.06x | 1.66x | 1.15x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.29x | 3.02x | 2.23x | 2.94x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.82x | 1.46x | 1.49x | 2.07x | 2.41x | 9/10 |
| replay_checked_native | 0.94x | 2.18x | 1.12x | 8.78x | 8.44x | 9.87x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
