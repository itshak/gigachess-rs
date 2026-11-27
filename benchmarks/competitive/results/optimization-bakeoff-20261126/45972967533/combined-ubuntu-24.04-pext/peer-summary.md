| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.71x | 1.96x | 1.71x | 2.62x | 2.02x | 30/30 |
| materialize | 0.97x | 2.18x | 1.61x | 1.77x | 2.70x | 1.89x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.06x | 1.61x | 1.27x | 2.48x | 1.98x | 29/30 |
| perft_full_d3 | 1.24x | 1.74x | 1.90x | 1.86x | 2.45x | 2.02x | 30/30 |
| fen_parse | 0.99x | 1.33x | 1.56x | 1.05x | 1.51x | 1.24x | 30/30 |
| fen_write_native_ep | 1.01x | 5.02x | 10.26x | 1.93x | 28.51x | 2.65x | 30/30 |
| snapshot | 0.99x | 1.82x | 1.16x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.05x | N/A | 2.09x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.06x | 1.35x | 1.69x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.13x | 88.75x | 154.78x | 203.14x | 178.40x | 5/5 |
| chess960_materialize | 1.01x | 1.79x | 1.69x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.57x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 22.42x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.14x | 6.26x | 2.32x | 1.29x | 2.91x | 2.59x | 10/10 |
| corpus_materialize | 1.01x | 1.44x | 1.32x | 1.07x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.34x | 2.98x | 2.27x | 2.97x | 1.58x | 10/10 |
| replay_trusted_native | 1.02x | 0.93x | 1.69x | 1.60x | 2.29x | 2.98x | 9/10 |
| replay_checked_native | 1.01x | 2.26x | 1.20x | 9.19x | 9.08x | 10.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
