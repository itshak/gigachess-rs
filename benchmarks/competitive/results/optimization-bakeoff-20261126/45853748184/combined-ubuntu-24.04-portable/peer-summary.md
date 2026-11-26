| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.86x | 1.64x | 1.44x | 1.98x | 1.41x | 30/30 |
| materialize | 1.00x | 2.30x | 1.97x | 1.87x | 3.01x | 2.02x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.01x | 1.59x | 1.18x | 2.08x | 1.80x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.61x | 1.42x | 1.86x | 1.61x | 30/30 |
| fen_parse | 1.02x | 1.61x | 1.43x | 1.23x | 1.50x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 2.99x | 9.99x | 1.80x | 27.94x | 2.58x | 30/30 |
| snapshot | 1.00x | 1.76x | 1.25x | 3.26x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.96x | 1.81x | N/A | 3.98x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.42x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.10x | 1.34x | 4.01x | 1.29x | 1.47x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.29x | 63.34x | 109.94x | 122.51x | 118.69x | 5/5 |
| chess960_materialize | 1.00x | 1.87x | 1.81x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.12x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 43.03x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 23.02x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.10x | 2.07x | 1.13x | 2.11x | 2.87x | 10/10 |
| corpus_materialize | 1.00x | 1.51x | 1.42x | 1.14x | 1.89x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 2.98x | 2.20x | 2.92x | 1.56x | 10/10 |
| replay_trusted_native | 0.94x | 0.86x | 1.59x | 1.52x | 2.11x | 2.02x | 9/10 |
| replay_checked_native | 0.99x | 1.91x | 1.09x | 8.15x | 7.93x | 9.07x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
