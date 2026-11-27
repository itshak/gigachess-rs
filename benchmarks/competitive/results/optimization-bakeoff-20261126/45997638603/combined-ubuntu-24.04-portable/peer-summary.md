| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 3.01x | 1.74x | 1.51x | 2.04x | 1.48x | 30/30 |
| materialize | 0.99x | 2.25x | 1.91x | 1.88x | 2.91x | 2.02x | 30/30 |
| perft_bulk_d4 | 1.06x | 3.22x | 1.70x | 1.27x | 2.15x | 1.93x | 29/30 |
| perft_full_d3 | 1.18x | 1.59x | 1.88x | 1.67x | 2.28x | 1.88x | 30/30 |
| fen_parse | 1.02x | 1.58x | 1.49x | 1.24x | 1.62x | 1.22x | 30/30 |
| fen_write_native_ep | 1.00x | 2.98x | 10.28x | 1.81x | 26.37x | 2.54x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.92x | N/A | 4.24x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.17x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.22x | 1.50x | 4.28x | 1.35x | 1.60x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.95x | 109.17x | 129.67x | 103.88x | 5/5 |
| chess960_materialize | 0.91x | 1.73x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.14x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 43.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 23.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.19x | 4.90x | 2.43x | 1.34x | 2.48x | 3.41x | 10/10 |
| corpus_materialize | 1.00x | 1.52x | 1.41x | 1.14x | 1.83x | 1.24x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.93x | 2.92x | 2.17x | 2.81x | 1.52x | 10/10 |
| replay_trusted_native | 1.00x | 0.91x | 1.68x | 1.60x | 2.23x | 2.04x | 9/10 |
| replay_checked_native | 1.00x | 2.09x | 1.17x | 8.91x | 8.67x | 9.84x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
