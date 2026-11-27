| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.35x | 1.66x | 1.78x | 2.16x | 2.15x | 30/30 |
| materialize | 0.99x | 1.94x | 1.53x | 1.34x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.82x | 1.57x | 1.41x | 2.00x | 2.05x | 30/30 |
| perft_full_d3 | 0.98x | 1.45x | 1.57x | 1.30x | 1.92x | 1.82x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.32x | 1.25x | 1.73x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.53x | 9.32x | 1.56x | 30.07x | 2.45x | 30/30 |
| snapshot | 1.00x | 1.06x | 0.99x | 1.47x | 0.99x | 0.95x | 18/30 |
| san_parse_all | 0.99x | 1.83x | N/A | 4.15x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.00x | N/A | 2.59x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.22x | 1.44x | 3.92x | 1.17x | 1.17x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.72x | 129.43x | 171.97x | 138.11x | 5/5 |
| chess960_materialize | 1.00x | 1.70x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.93x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 25.18x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.67x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.40x | 1.72x | 1.35x | 1.98x | 2.21x | 10/10 |
| corpus_materialize | 0.99x | 1.59x | 1.53x | 1.23x | 1.89x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.09x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.62x | 1.66x | 1.99x | 2.20x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.13x | 7.58x | 6.75x | 8.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
