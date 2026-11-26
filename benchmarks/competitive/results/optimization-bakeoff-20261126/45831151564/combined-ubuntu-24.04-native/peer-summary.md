| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.44x | 1.51x | 1.45x | 2.12x | 1.90x | 30/30 |
| materialize | 0.98x | 1.98x | 1.54x | 1.09x | 2.17x | 1.40x | 29/30 |
| perft_bulk_d4 | 0.98x | 3.76x | 1.43x | 1.12x | 2.14x | 2.05x | 29/30 |
| perft_full_d3 | 0.98x | 1.36x | 1.59x | 1.19x | 2.05x | 2.02x | 30/30 |
| fen_parse | 1.00x | 1.43x | 1.62x | 1.32x | 2.12x | 1.36x | 30/30 |
| fen_write_native_ep | 1.08x | 4.18x | 8.64x | 1.55x | 25.30x | 2.00x | 30/30 |
| snapshot | 0.98x | 2.48x | 1.49x | 8.19x | 1.49x | 1.49x | 30/30 |
| san_parse_all | 0.99x | 1.80x | N/A | 3.94x | N/A | N/A | 12/12 |
| san_render_native | 0.92x | 9.91x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.87x | 1.37x | 3.92x | 1.38x | 1.87x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.25x | 85.97x | 149.92x | 224.59x | 157.04x | 5/5 |
| chess960_materialize | 0.94x | 1.57x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.70x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 34.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.82x | 22.73x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.32x | 1.61x | 1.15x | 2.03x | 1.99x | 9/10 |
| corpus_materialize | 0.98x | 1.58x | 1.45x | 1.06x | 2.04x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.36x | 2.98x | 2.54x | 2.98x | 1.65x | 10/10 |
| replay_trusted_native | 0.98x | 0.71x | 1.31x | 1.47x | 2.35x | 2.78x | 8/10 |
| replay_checked_native | 1.04x | 2.73x | 1.15x | 9.05x | 9.04x | 11.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
