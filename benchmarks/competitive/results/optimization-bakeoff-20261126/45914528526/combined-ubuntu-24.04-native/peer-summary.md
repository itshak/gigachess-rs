| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.22x | 1.81x | 1.53x | 2.35x | 1.88x | 30/30 |
| materialize | 0.99x | 2.20x | 1.67x | 1.70x | 2.54x | 2.03x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.71x | 1.53x | 1.15x | 2.28x | 1.84x | 29/30 |
| perft_full_d3 | 1.01x | 1.35x | 1.60x | 1.51x | 1.96x | 1.73x | 30/30 |
| fen_parse | 0.92x | 1.30x | 1.24x | 1.07x | 1.34x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 4.79x | 10.14x | 2.13x | 28.28x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.82x | 1.15x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.01x | 2.08x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.20x | N/A | 2.19x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.41x | 4.16x | 1.35x | 2.26x | 30/30 |
| make_unmake_full | 0.95x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 0.89x | 1.12x | 90.89x | 166.44x | 190.00x | 173.65x | 5/5 |
| chess960_materialize | 1.00x | 1.77x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.53x | 1.51x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.92x | 36.38x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.15x | 22.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.25x | 1.98x | 1.09x | 2.40x | 2.23x | 9/10 |
| corpus_materialize | 1.01x | 1.40x | 1.29x | 1.07x | 1.72x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.23x | 2.99x | 2.23x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 1.05x | 0.93x | 1.61x | 1.62x | 2.27x | 2.69x | 9/10 |
| replay_checked_native | 1.00x | 2.22x | 1.17x | 9.38x | 8.72x | 10.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
