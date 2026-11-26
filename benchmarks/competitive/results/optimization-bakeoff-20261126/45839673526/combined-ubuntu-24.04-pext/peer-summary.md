| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.29x | 1.77x | 1.57x | 2.18x | 1.86x | 30/30 |
| materialize | 0.97x | 2.03x | 1.56x | 1.65x | 2.38x | 1.60x | 30/30 |
| perft_bulk_d4 | 0.95x | 3.67x | 1.44x | 1.13x | 2.07x | 1.78x | 29/30 |
| perft_full_d3 | 0.88x | 1.15x | 1.38x | 1.24x | 1.71x | 1.39x | 30/30 |
| fen_parse | 1.00x | 1.15x | 1.30x | 0.97x | 1.47x | 1.23x | 25/30 |
| fen_write_native_ep | 0.99x | 4.99x | 10.51x | 1.95x | 31.02x | 2.69x | 30/30 |
| snapshot | 0.68x | 1.34x | 0.95x | 3.02x | 0.95x | 1.00x | 16/30 |
| san_parse_all | 0.96x | 1.74x | N/A | 4.03x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.70x | N/A | 1.96x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.06x | 1.32x | 3.90x | 1.33x | 1.57x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.11x | N/A | N/A | 6/6 |
| startpos | 0.77x | 0.91x | 63.98x | 102.39x | 145.43x | 122.12x | 4/5 |
| chess960_materialize | 0.87x | 1.51x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.33x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 40.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.84x | 22.69x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.77x | 1.85x | 0.94x | 2.12x | 2.01x | 8/10 |
| corpus_materialize | 1.00x | 1.44x | 1.31x | 1.09x | 1.72x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.24x | 2.97x | 2.20x | 2.85x | 1.54x | 10/10 |
| replay_trusted_native | 0.94x | 0.83x | 1.49x | 1.48x | 2.10x | 2.37x | 9/10 |
| replay_checked_native | 0.94x | 2.03x | 1.02x | 8.19x | 8.04x | 9.16x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
