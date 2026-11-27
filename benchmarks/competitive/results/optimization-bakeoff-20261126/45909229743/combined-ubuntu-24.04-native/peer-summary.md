| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 4.59x | 1.95x | 1.63x | 2.50x | 1.97x | 30/30 |
| materialize | 1.08x | 2.27x | 1.68x | 1.73x | 2.60x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.98x | 1.63x | 1.25x | 2.44x | 1.95x | 29/30 |
| perft_full_d3 | 1.24x | 1.68x | 1.97x | 1.86x | 2.43x | 2.06x | 30/30 |
| fen_parse | 0.92x | 1.25x | 1.48x | 0.99x | 1.53x | 1.06x | 28/30 |
| fen_write_native_ep | 0.99x | 4.47x | 10.02x | 2.15x | 27.74x | 2.70x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 1.93x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.05x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.14x | 1.34x | 4.06x | 1.34x | 1.70x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.13x | 88.34x | 153.88x | 254.37x | 162.27x | 5/5 |
| chess960_materialize | 1.07x | 1.72x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.52x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.96x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.18x | 23.48x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.23x | 6.53x | 2.44x | 1.34x | 2.95x | 2.68x | 10/10 |
| corpus_materialize | 1.01x | 1.40x | 1.29x | 1.05x | 1.70x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.25x | 2.91x | 2.22x | 2.84x | 1.55x | 10/10 |
| replay_trusted_native | 1.06x | 0.93x | 1.69x | 1.61x | 2.28x | 2.68x | 9/10 |
| replay_checked_native | 1.02x | 2.18x | 1.23x | 9.22x | 9.01x | 10.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
