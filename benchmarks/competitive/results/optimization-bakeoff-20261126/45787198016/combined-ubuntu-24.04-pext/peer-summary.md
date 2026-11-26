| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.23x | 1.77x | 1.54x | 2.37x | 1.81x | 30/30 |
| materialize | 1.05x | 2.17x | 1.61x | 1.69x | 2.58x | 1.88x | 30/30 |
| perft_bulk_d4 | 0.98x | 4.22x | 1.57x | 1.23x | 2.46x | 1.94x | 30/30 |
| perft_full_d3 | 0.96x | 2.13x | 1.54x | 1.51x | 1.99x | 1.58x | 30/30 |
| fen_parse | 0.92x | 1.19x | 1.41x | 0.94x | 1.29x | 1.01x | 20/30 |
| fen_write_native_ep | 1.00x | 5.09x | 10.41x | 2.13x | 28.16x | 2.72x | 30/30 |
| snapshot | 0.67x | 1.65x | 1.33x | 2.80x | 1.33x | 1.23x | 30/30 |
| san_parse_all | 1.08x | 1.82x | N/A | 4.18x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 8.49x | N/A | 1.99x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.57x | 0.98x | 3.00x | 0.97x | 1.22x | 24/30 |
| make_unmake_full | 0.90x | N/A | N/A | 1.02x | N/A | N/A | 4/6 |
| startpos | 0.89x | 1.50x | 78.90x | 138.34x | 175.19x | 145.57x | 5/5 |
| chess960_materialize | 0.92x | 1.59x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.93x | 2.21x | 1.29x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.04x | 20.08x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.91x | 2.52x | 1.43x | 3.12x | 2.87x | 10/10 |
| corpus_materialize | 1.02x | 1.49x | 1.37x | 1.14x | 1.80x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.31x | 2.97x | 2.28x | 2.89x | 1.57x | 10/10 |
| replay_trusted_native | 0.77x | 0.56x | 1.01x | 0.98x | 1.41x | 1.61x | 6/10 |
| replay_checked_native | 0.73x | 1.48x | 0.79x | 6.16x | 5.87x | 6.99x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
