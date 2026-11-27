| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.24x | 1.82x | 1.54x | 2.32x | 1.86x | 30/30 |
| materialize | 1.00x | 2.24x | 1.70x | 1.73x | 2.63x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.72x | 1.53x | 1.16x | 2.25x | 1.83x | 29/30 |
| perft_full_d3 | 1.00x | 1.34x | 1.59x | 1.49x | 1.92x | 1.64x | 30/30 |
| fen_parse | 0.93x | 1.30x | 1.27x | 1.00x | 1.40x | 1.11x | 28/30 |
| fen_write_native_ep | 0.96x | 4.73x | 9.70x | 2.04x | 27.04x | 2.57x | 30/30 |
| snapshot | 1.01x | 1.67x | 1.33x | 4.01x | 1.34x | 1.34x | 30/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.52x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.13x | 1.35x | 4.24x | 1.27x | 1.76x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 99.81x | 171.69x | 215.27x | 181.00x | 5/5 |
| chess960_materialize | 1.00x | 1.75x | 1.76x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.77x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.19x | 23.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.29x | 2.00x | 1.08x | 2.38x | 2.18x | 9/10 |
| corpus_materialize | 1.00x | 1.39x | 1.29x | 1.07x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.28x | 3.03x | 2.20x | 2.94x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.58x | 1.53x | 2.13x | 2.53x | 9/10 |
| replay_checked_native | 1.01x | 2.19x | 1.15x | 9.32x | 8.91x | 10.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
