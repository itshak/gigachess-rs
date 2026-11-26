| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.69x | 1.55x | 1.35x | 1.91x | 1.33x | 29/30 |
| materialize | 1.00x | 2.17x | 1.72x | 1.73x | 2.64x | 2.03x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.71x | 1.43x | 1.07x | 1.79x | 1.63x | 29/30 |
| perft_full_d3 | 1.00x | 1.50x | 1.70x | 1.53x | 2.10x | 1.75x | 30/30 |
| fen_parse | 1.00x | 1.59x | 1.51x | 1.25x | 1.58x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 3.00x | 9.77x | 1.81x | 26.81x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.01x | 1.91x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 8.00x | N/A | 1.80x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.98x | 1.34x | 3.82x | 1.21x | 1.42x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.42x | 58.28x | 104.70x | 124.06x | 108.29x | 5/5 |
| chess960_materialize | 0.97x | 1.56x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 1.95x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.09x | 43.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.14x | 20.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 3.64x | 1.84x | 0.99x | 1.92x | 2.54x | 9/10 |
| corpus_materialize | 1.01x | 1.45x | 1.35x | 1.08x | 1.79x | 1.20x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.90x | 2.80x | 2.12x | 2.79x | 1.50x | 10/10 |
| replay_trusted_native | 1.02x | 0.87x | 1.55x | 1.54x | 2.13x | 1.98x | 9/10 |
| replay_checked_native | 0.98x | 1.99x | 1.08x | 8.49x | 8.41x | 9.46x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
