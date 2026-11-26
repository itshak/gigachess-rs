| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.90x | 1.65x | 1.44x | 1.92x | 1.38x | 30/30 |
| materialize | 0.93x | 2.14x | 1.72x | 1.88x | 2.64x | 1.97x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.99x | 1.59x | 1.19x | 1.99x | 1.78x | 29/30 |
| perft_full_d3 | 1.00x | 1.36x | 1.57x | 1.41x | 1.92x | 1.58x | 30/30 |
| fen_parse | 1.01x | 1.47x | 1.45x | 1.20x | 1.59x | 1.17x | 30/30 |
| fen_write_native_ep | 1.00x | 3.11x | 9.97x | 1.87x | 27.01x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.74x | 1.24x | 3.24x | 1.24x | 1.00x | 25/30 |
| san_parse_all | 1.03x | 1.94x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.23x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.22x | 1.47x | 4.27x | 1.36x | 1.58x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 57.28x | 109.14x | 129.17x | 98.42x | 5/5 |
| chess960_materialize | 1.00x | 1.81x | 1.76x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.13x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 41.00x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.06x | 1.02x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.24x | 24.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 4.14x | 2.05x | 1.14x | 2.11x | 2.85x | 10/10 |
| corpus_materialize | 1.01x | 1.51x | 1.40x | 1.15x | 1.83x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.91x | 2.85x | 2.15x | 2.76x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.88x | 1.64x | 1.66x | 2.28x | 2.05x | 9/10 |
| replay_checked_native | 1.08x | 2.12x | 1.16x | 8.86x | 8.76x | 9.90x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
