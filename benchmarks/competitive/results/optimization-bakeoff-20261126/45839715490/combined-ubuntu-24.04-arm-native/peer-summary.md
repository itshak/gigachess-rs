| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.79x | 2.17x | 2.16x | 30/30 |
| materialize | 0.99x | 1.94x | 1.54x | 1.33x | 2.14x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 0.92x | 1.00x | 0.83x | 1.23x | 1.16x | 17/30 |
| fen_parse | 1.01x | 1.24x | 1.33x | 1.24x | 1.72x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 9.39x | 1.61x | 30.14x | 2.47x | 30/30 |
| snapshot | 1.01x | 1.17x | 1.01x | 1.52x | 1.00x | 0.94x | 21/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.76x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.47x | 4.07x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.37x | 130.14x | 175.41x | 142.29x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 24.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.99x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.43x | 1.70x | 1.39x | 2.00x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.53x | 1.22x | 1.91x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.12x | 2.94x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.67x | 2.00x | 2.22x | 2.25x | 8/10 |
| replay_checked_native | 1.01x | 2.20x | 1.06x | 7.12x | 6.38x | 8.17x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
