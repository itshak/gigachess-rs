| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.31x | 1.51x | 1.48x | 2.14x | 1.91x | 30/30 |
| materialize | 1.03x | 1.90x | 1.32x | 1.19x | 2.19x | 1.46x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.56x | 1.38x | 1.20x | 2.00x | 1.78x | 30/30 |
| perft_full_d3 | 1.01x | 1.50x | 1.39x | 1.23x | 1.82x | 1.78x | 30/30 |
| fen_parse | 1.13x | 1.45x | 1.56x | 1.46x | 2.38x | 1.52x | 30/30 |
| fen_write_native_ep | 1.05x | 4.05x | 8.08x | 1.56x | 31.92x | 2.01x | 30/30 |
| snapshot | 1.03x | 1.93x | 0.97x | 4.01x | 1.00x | 0.97x | 18/30 |
| san_parse_all | 1.07x | 2.10x | N/A | 5.63x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.61x | N/A | 3.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 2.27x | 1.38x | 9.21x | 1.45x | 1.62x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.04x | 1.25x | 78.40x | 156.24x | 231.58x | 165.59x | 5/5 |
| chess960_materialize | 1.08x | 1.55x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.08x | 2.70x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 26.31x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 20.76x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.83x | 1.62x | 1.24x | 2.23x | 2.16x | 10/10 |
| corpus_materialize | 1.07x | 1.48x | 1.19x | 1.09x | 1.85x | 1.29x | 10/10 |
| corpus_uci_sorted | 0.94x | 1.66x | 2.08x | 1.69x | 2.01x | 1.32x | 10/10 |
| replay_trusted_native | 1.00x | 0.83x | 1.57x | 2.05x | 2.57x | 3.01x | 9/10 |
| replay_checked_native | 1.03x | 2.20x | 0.99x | 7.64x | 7.59x | 8.92x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
