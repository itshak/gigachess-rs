| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 3.92x | 1.49x | 1.61x | 1.94x | 1.92x | 30/30 |
| materialize | 1.01x | 1.86x | 1.46x | 1.27x | 2.03x | 1.40x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.44x | 1.40x | 1.27x | 1.79x | 1.83x | 30/30 |
| perft_full_d3 | 0.99x | 1.15x | 1.25x | 1.04x | 1.50x | 1.45x | 29/30 |
| fen_parse | 0.99x | 1.25x | 1.33x | 1.30x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.33x | 1.58x | 29.53x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.14x | 1.17x | 1.64x | 1.20x | 1.03x | 30/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.74x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.12x | 1.33x | 3.61x | 1.07x | 1.07x | 27/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.44x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.16x | 67.36x | 112.34x | 148.13x | 117.38x | 5/5 |
| chess960_materialize | 1.05x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.67x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.24x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.83x | 3.95x | 1.51x | 1.19x | 1.76x | 1.96x | 10/10 |
| corpus_materialize | 0.97x | 1.54x | 1.46x | 1.17x | 1.82x | 1.32x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.10x | 2.91x | 2.08x | 2.84x | 1.45x | 10/10 |
| replay_trusted_native | 0.98x | 0.49x | 1.29x | 1.56x | 1.74x | 1.80x | 8/10 |
| replay_checked_native | 1.00x | 2.04x | 0.99x | 6.64x | 5.96x | 7.74x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
