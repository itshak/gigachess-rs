| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.06x | 1.55x | 1.66x | 2.01x | 2.00x | 30/30 |
| materialize | 1.06x | 1.93x | 1.53x | 1.32x | 2.12x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.96x | 3.59x | 1.47x | 1.32x | 1.87x | 1.92x | 30/30 |
| perft_full_d3 | 1.05x | 1.22x | 1.33x | 1.10x | 1.62x | 1.53x | 29/30 |
| fen_parse | 0.99x | 1.24x | 1.35x | 1.29x | 1.76x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.41x | 9.20x | 1.60x | 29.69x | 2.37x | 30/30 |
| snapshot | 0.97x | 1.10x | 1.00x | 1.45x | 1.02x | 0.95x | 20/30 |
| san_parse_all | 0.99x | 1.83x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 11.59x | N/A | 2.52x | N/A | N/A | 12/12 |
| apply_children_full | 1.06x | 1.27x | 1.51x | 4.09x | 1.23x | 1.23x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.48x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.31x | 76.33x | 130.01x | 172.52x | 139.02x | 5/5 |
| chess960_materialize | 1.11x | 1.68x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.09x | 2.87x | 1.44x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 18.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.86x | 4.11x | 1.58x | 1.26x | 1.89x | 2.02x | 10/10 |
| corpus_materialize | 1.03x | 1.62x | 1.54x | 1.22x | 1.92x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.94x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.05x | 0.55x | 1.43x | 1.76x | 1.95x | 1.99x | 8/10 |
| replay_checked_native | 1.04x | 2.11x | 1.02x | 6.84x | 6.19x | 7.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
