| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.64x | 1.51x | 1.32x | 1.75x | 1.30x | 29/30 |
| materialize | 0.94x | 1.99x | 1.63x | 1.73x | 2.48x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.94x | 1.56x | 1.15x | 1.93x | 1.77x | 30/30 |
| perft_full_d3 | 1.00x | 1.56x | 1.78x | 1.61x | 2.14x | 1.82x | 30/30 |
| fen_parse | 0.94x | 1.46x | 1.37x | 1.13x | 1.43x | 1.18x | 30/30 |
| fen_write_native_ep | 1.03x | 3.01x | 9.70x | 1.93x | 27.42x | 2.63x | 30/30 |
| snapshot | 1.00x | 1.46x | 1.12x | 2.91x | 1.12x | 1.00x | 29/30 |
| san_parse_all | 0.92x | 1.77x | N/A | 4.12x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.18x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.87x | 1.20x | 3.52x | 1.17x | 1.32x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.29x | 57.56x | 109.44x | 129.99x | 113.58x | 5/5 |
| chess960_materialize | 0.99x | 1.62x | 1.65x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 1.88x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.90x | 39.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.99x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 21.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.62x | 2.33x | 1.25x | 2.37x | 3.24x | 10/10 |
| corpus_materialize | 1.02x | 1.54x | 1.43x | 1.17x | 1.88x | 1.27x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.93x | 2.90x | 2.19x | 2.87x | 1.55x | 10/10 |
| replay_trusted_native | 1.01x | 0.73x | 1.32x | 1.35x | 1.82x | 1.69x | 8/10 |
| replay_checked_native | 1.00x | 1.98x | 1.10x | 8.44x | 8.38x | 9.45x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
