| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.63x | 1.67x | 1.61x | 2.48x | 2.00x | 30/30 |
| materialize | 1.01x | 2.00x | 1.69x | 1.26x | 2.29x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.94x | 1.58x | 1.41x | 2.40x | 1.97x | 30/30 |
| perft_full_d3 | 1.04x | 1.15x | 1.31x | 1.14x | 1.67x | 1.60x | 30/30 |
| fen_parse | 1.02x | 1.39x | 1.38x | 1.13x | 2.17x | 1.31x | 30/30 |
| fen_write_native_ep | 0.97x | 3.96x | 9.18x | 1.68x | 27.05x | 2.03x | 30/30 |
| snapshot | 0.98x | 1.95x | 1.23x | 3.48x | 1.23x | 1.23x | 30/30 |
| san_parse_all | 1.00x | 2.08x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.40x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.27x | 1.29x | 4.70x | 1.45x | 1.74x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.29x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 77.66x | 134.66x | 224.40x | 155.20x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.77x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 29.72x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 20.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.30x | 1.76x | 1.33x | 2.54x | 2.33x | 10/10 |
| corpus_materialize | 1.02x | 1.60x | 1.55x | 1.16x | 2.03x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.40x | 3.24x | 2.44x | 3.04x | 1.71x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.59x | 1.90x | 2.51x | 2.85x | 9/10 |
| replay_checked_native | 1.01x | 2.30x | 1.01x | 7.57x | 7.69x | 9.02x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
