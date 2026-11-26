| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.42x | 1.68x | 1.82x | 2.17x | 2.17x | 30/30 |
| materialize | 1.00x | 1.96x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.84x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.61x | 1.34x | 1.97x | 1.86x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.30x | 1.25x | 1.73x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 9.36x | 1.60x | 29.91x | 2.46x | 30/30 |
| snapshot | 1.00x | 1.10x | 1.01x | 1.47x | 1.02x | 0.95x | 20/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.41x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.04x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.13x | 129.07x | 173.20x | 139.87x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.41x | 1.69x | 1.37x | 1.99x | 2.21x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.52x | 1.23x | 1.91x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.92x | 2.08x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.64x | 1.64x | 2.00x | 2.26x | 2.27x | 8/10 |
| replay_checked_native | 0.99x | 2.29x | 1.12x | 7.57x | 6.75x | 8.75x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
