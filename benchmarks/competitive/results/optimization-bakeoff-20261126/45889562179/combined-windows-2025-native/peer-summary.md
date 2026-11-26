| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.74x | 1.70x | 1.62x | 2.34x | 2.07x | 30/30 |
| materialize | 0.98x | 1.93x | 1.32x | 1.22x | 2.23x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.95x | 1.59x | 1.34x | 2.25x | 2.04x | 30/30 |
| perft_full_d3 | 0.99x | 1.39x | 1.49x | 1.35x | 1.94x | 1.87x | 30/30 |
| fen_parse | 1.01x | 1.28x | 1.38x | 1.39x | 2.21x | 1.46x | 30/30 |
| fen_write_native_ep | 1.01x | 4.32x | 8.10x | 1.56x | 32.31x | 2.01x | 30/30 |
| snapshot | 1.02x | 2.05x | 1.00x | 4.10x | 1.06x | 1.02x | 26/30 |
| san_parse_all | 1.01x | 2.09x | N/A | 5.72x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.20x | N/A | 3.92x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.33x | 1.41x | 9.85x | 1.49x | 1.69x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.72x | 161.52x | 233.19x | 173.62x | 5/5 |
| chess960_materialize | 1.05x | 1.64x | 1.36x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.82x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.79x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.01x | 19.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 5.12x | 1.76x | 1.32x | 2.34x | 2.29x | 10/10 |
| corpus_materialize | 1.04x | 1.54x | 1.25x | 1.14x | 2.02x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.77x | 2.21x | 1.82x | 2.18x | 1.41x | 10/10 |
| replay_trusted_native | 1.00x | 0.85x | 1.59x | 2.08x | 2.56x | 2.95x | 9/10 |
| replay_checked_native | 1.01x | 2.46x | 1.10x | 8.52x | 8.21x | 9.91x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
