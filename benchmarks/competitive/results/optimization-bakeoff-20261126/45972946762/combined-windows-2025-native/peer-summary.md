| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.81x | 1.78x | 1.71x | 2.75x | 2.16x | 30/30 |
| materialize | 1.00x | 1.99x | 1.32x | 1.15x | 2.31x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.98x | 1.62x | 1.44x | 2.59x | 2.23x | 30/30 |
| perft_full_d3 | 0.98x | 1.48x | 1.52x | 1.28x | 2.06x | 1.82x | 30/30 |
| fen_parse | 1.07x | 1.57x | 1.61x | 1.59x | 2.59x | 1.63x | 30/30 |
| fen_write_native_ep | 1.01x | 3.94x | 7.58x | 1.59x | 31.32x | 2.00x | 30/30 |
| snapshot | 1.00x | 1.79x | 1.26x | 4.13x | 1.26x | 1.16x | 30/30 |
| san_parse_all | 0.99x | 1.94x | N/A | 4.73x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.16x | N/A | 3.26x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.97x | 1.32x | 8.23x | 1.55x | 1.74x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.00x | 83.55x | 137.23x | 213.10x | 144.03x | 4/5 |
| chess960_materialize | 1.00x | 1.67x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.93x | 1.61x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 20.23x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.14x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 4.42x | 1.80x | 1.33x | 2.38x | 2.14x | 10/10 |
| corpus_materialize | 1.00x | 1.87x | 1.45x | 1.19x | 2.82x | 1.52x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.89x | 2.41x | 2.00x | 2.30x | 1.56x | 10/10 |
| replay_trusted_native | 0.98x | 1.10x | 1.92x | 2.54x | 3.87x | 4.75x | 10/10 |
| replay_checked_native | 1.01x | 2.46x | 1.07x | 6.12x | 7.22x | 8.26x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
