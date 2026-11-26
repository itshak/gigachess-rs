| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.94x | 3.70x | 1.58x | 1.34x | 2.07x | 1.61x | 30/30 |
| materialize | 0.96x | 2.02x | 1.53x | 1.58x | 2.49x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.97x | 3.17x | 1.30x | 0.99x | 1.95x | 1.55x | 26/30 |
| perft_full_d3 | 0.96x | 1.38x | 1.57x | 1.52x | 1.99x | 1.69x | 30/30 |
| fen_parse | 1.05x | 1.45x | 1.36x | 1.13x | 1.55x | 1.21x | 29/30 |
| fen_write_native_ep | 0.92x | 4.12x | 9.45x | 1.90x | 27.81x | 2.36x | 30/30 |
| snapshot | 0.55x | 1.34x | 1.08x | 2.28x | 1.08x | 1.00x | 25/30 |
| san_parse_all | 0.95x | 1.86x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.03x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 2.01x | 1.31x | 3.82x | 1.28x | 1.63x | 30/30 |
| make_unmake_full | 0.92x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 1.00x | 1.49x | 78.70x | 140.63x | 182.30x | 151.89x | 5/5 |
| chess960_materialize | 0.94x | 1.60x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.24x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 38.96x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.89x | 19.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.97x | 4.12x | 1.55x | 0.85x | 1.88x | 1.73x | 8/10 |
| corpus_materialize | 1.00x | 1.35x | 1.26x | 1.03x | 1.67x | 1.14x | 9/10 |
| corpus_uci_sorted | 1.05x | 2.27x | 3.08x | 2.29x | 3.01x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.86x | 1.53x | 1.60x | 2.22x | 2.59x | 9/10 |
| replay_checked_native | 0.96x | 1.90x | 0.99x | 8.09x | 7.91x | 9.30x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
