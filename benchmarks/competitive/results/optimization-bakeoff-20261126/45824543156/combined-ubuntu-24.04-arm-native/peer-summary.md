| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.90x | 1.49x | 1.60x | 1.94x | 1.92x | 30/30 |
| materialize | 1.02x | 1.85x | 1.45x | 1.27x | 2.04x | 1.40x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.41x | 1.40x | 1.27x | 1.78x | 1.83x | 30/30 |
| perft_full_d3 | 0.98x | 1.16x | 1.26x | 1.04x | 1.54x | 1.46x | 29/30 |
| fen_parse | 0.98x | 1.22x | 1.31x | 1.24x | 1.72x | 1.38x | 30/30 |
| fen_write_native_ep | 1.00x | 4.56x | 9.27x | 1.58x | 30.19x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.04x | 1.06x | 1.45x | 1.07x | 0.96x | 23/30 |
| san_parse_all | 1.01x | 1.84x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.69x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 0.89x | 1.24x | 1.48x | 3.99x | 1.19x | 1.20x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.28x | 127.39x | 175.04x | 141.47x | 5/5 |
| chess960_materialize | 1.01x | 1.59x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.61x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.94x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.05x | 19.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.91x | 1.52x | 1.22x | 1.82x | 1.95x | 10/10 |
| corpus_materialize | 1.01x | 1.54x | 1.48x | 1.18x | 1.87x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.91x | 2.08x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 1.00x | 0.62x | 1.62x | 1.96x | 2.18x | 2.23x | 8/10 |
| replay_checked_native | 1.01x | 2.18x | 1.07x | 7.13x | 6.48x | 8.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
