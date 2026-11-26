| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.01x | 1.73x | 1.46x | 2.27x | 1.79x | 30/30 |
| materialize | 1.05x | 2.13x | 1.60x | 1.66x | 2.61x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.59x | 1.21x | 2.40x | 1.91x | 30/30 |
| perft_full_d3 | 1.01x | 1.51x | 1.76x | 1.69x | 2.37x | 1.86x | 30/30 |
| fen_parse | 1.08x | 1.25x | 1.31x | 1.14x | 1.44x | 1.12x | 30/30 |
| fen_write_native_ep | 1.00x | 4.65x | 10.14x | 1.89x | 29.60x | 2.20x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 26/30 |
| san_parse_all | 1.04x | 1.96x | N/A | 4.48x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.11x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.80x | 1.08x | 3.23x | 1.07x | 1.30x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.14x | N/A | N/A | 6/6 |
| startpos | 0.80x | 1.00x | 79.20x | 146.14x | 171.32x | 141.07x | 5/5 |
| chess960_materialize | 1.13x | 1.73x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.27x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 39.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.91x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.17x | 22.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.44x | 2.43x | 1.33x | 2.95x | 2.70x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.33x | 1.08x | 1.76x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.21x | 2.95x | 2.22x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.70x | 1.28x | 1.32x | 1.80x | 2.11x | 8/10 |
| replay_checked_native | 1.01x | 1.99x | 1.11x | 8.46x | 8.27x | 9.80x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
