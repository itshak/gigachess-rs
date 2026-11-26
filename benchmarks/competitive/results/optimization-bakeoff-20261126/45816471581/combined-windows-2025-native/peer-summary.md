| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.34x | 1.54x | 1.47x | 2.26x | 1.87x | 30/30 |
| materialize | 1.02x | 1.91x | 1.32x | 1.20x | 2.29x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.08x | 3.60x | 1.45x | 1.22x | 2.05x | 1.84x | 30/30 |
| perft_full_d3 | 1.02x | 1.37x | 1.44x | 1.29x | 1.90x | 1.84x | 30/30 |
| fen_parse | 0.89x | 1.14x | 1.29x | 1.27x | 1.99x | 1.32x | 30/30 |
| fen_write_native_ep | 1.01x | 4.15x | 8.38x | 1.57x | 32.56x | 2.00x | 30/30 |
| snapshot | 0.99x | 1.97x | 1.01x | 4.09x | 1.00x | 1.02x | 22/30 |
| san_parse_all | 1.05x | 2.06x | N/A | 5.44x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 10.78x | N/A | 3.75x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.13x | 1.29x | 8.77x | 1.36x | 1.53x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 90.79x | 168.90x | 236.78x | 176.14x | 5/5 |
| chess960_materialize | 1.01x | 1.66x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.84x | 2.32x | 1.24x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 27.54x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.88x | 18.66x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.91x | 4.56x | 1.63x | 1.17x | 2.09x | 2.06x | 9/10 |
| corpus_materialize | 0.99x | 1.48x | 1.20x | 1.12x | 1.91x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.05x | 1.82x | 2.31x | 1.86x | 2.23x | 1.45x | 10/10 |
| replay_trusted_native | 0.97x | 0.71x | 1.34x | 1.83x | 2.25x | 2.58x | 8/10 |
| replay_checked_native | 1.01x | 2.24x | 1.04x | 7.83x | 7.76x | 9.58x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
