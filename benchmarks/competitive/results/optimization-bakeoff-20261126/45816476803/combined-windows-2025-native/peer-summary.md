| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.59x | 1.52x | 1.42x | 2.14x | 1.90x | 30/30 |
| materialize | 0.98x | 2.04x | 1.38x | 1.18x | 2.35x | 1.48x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.86x | 1.41x | 1.27x | 2.18x | 2.02x | 30/30 |
| perft_full_d3 | 1.01x | 1.40x | 1.50x | 1.27x | 2.07x | 2.09x | 30/30 |
| fen_parse | 0.92x | 1.32x | 1.49x | 1.48x | 2.14x | 1.52x | 30/30 |
| fen_write_native_ep | 1.02x | 3.83x | 7.17x | 1.51x | 32.02x | 2.06x | 30/30 |
| snapshot | 0.95x | 2.42x | 1.45x | 5.27x | 1.45x | 1.45x | 30/30 |
| san_parse_all | 0.99x | 1.96x | N/A | 4.62x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.55x | N/A | 3.73x | N/A | N/A | 12/12 |
| apply_children_full | 1.09x | 1.78x | 1.31x | 8.35x | 1.34x | 1.82x | 30/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 0.95x | 1.25x | 88.51x | 169.65x | 238.45x | 178.80x | 5/5 |
| chess960_materialize | 0.96x | 1.61x | 1.33x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 2.75x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 32.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 22.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 5.56x | 1.61x | 1.06x | 2.10x | 2.02x | 9/10 |
| corpus_materialize | 1.07x | 1.64x | 1.24x | 1.12x | 2.19x | 1.33x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.65x | 1.96x | 1.76x | 1.96x | 1.39x | 10/10 |
| replay_trusted_native | 1.09x | 0.66x | 1.31x | 1.53x | 2.26x | 2.75x | 8/10 |
| replay_checked_native | 1.01x | 2.31x | 0.97x | 7.58x | 7.41x | 9.33x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
