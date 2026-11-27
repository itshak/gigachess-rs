| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.30x | 1.85x | 1.57x | 2.69x | 1.92x | 30/30 |
| materialize | 0.95x | 2.09x | 1.57x | 1.71x | 2.51x | 1.95x | 30/30 |
| perft_bulk_d4 | 1.05x | 3.91x | 1.61x | 1.22x | 2.57x | 1.94x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.54x | 1.48x | 1.96x | 1.66x | 30/30 |
| fen_parse | 0.93x | 1.31x | 1.24x | 1.10x | 1.42x | 1.07x | 30/30 |
| fen_write_native_ep | 0.99x | 4.86x | 9.98x | 2.09x | 27.98x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 2.08x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.23x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.15x | 1.34x | 4.06x | 1.34x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 89.06x | 169.19x | 189.59x | 156.11x | 5/5 |
| chess960_materialize | 0.99x | 1.75x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.47x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.26x | 23.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.67x | 2.13x | 1.17x | 2.60x | 2.40x | 10/10 |
| corpus_materialize | 1.02x | 1.41x | 1.30x | 1.07x | 1.73x | 1.18x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.26x | 3.00x | 2.23x | 2.92x | 1.55x | 10/10 |
| replay_trusted_native | 0.99x | 0.87x | 1.54x | 1.53x | 2.18x | 2.51x | 9/10 |
| replay_checked_native | 1.01x | 2.24x | 1.20x | 9.22x | 8.88x | 10.56x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
