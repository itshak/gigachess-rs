| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.34x | 1.51x | 1.46x | 2.11x | 1.88x | 30/30 |
| materialize | 1.03x | 1.93x | 1.32x | 1.20x | 2.23x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.07x | 3.60x | 1.41x | 1.21x | 2.02x | 1.82x | 30/30 |
| perft_full_d3 | 1.00x | 1.33x | 1.40x | 1.26x | 1.83x | 1.79x | 30/30 |
| fen_parse | 1.05x | 1.34x | 1.49x | 1.32x | 2.29x | 1.48x | 30/30 |
| fen_write_native_ep | 1.06x | 4.33x | 8.60x | 1.64x | 32.23x | 2.08x | 30/30 |
| snapshot | 1.01x | 1.96x | 1.09x | 4.15x | 1.05x | 1.01x | 23/30 |
| san_parse_all | 1.02x | 2.06x | N/A | 5.54x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.08x | N/A | 3.88x | N/A | N/A | 12/12 |
| apply_children_full | 1.07x | 2.32x | 1.40x | 9.56x | 1.49x | 1.73x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.35x | 153.42x | 238.10x | 174.07x | 5/5 |
| chess960_materialize | 1.01x | 1.54x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.18x | 2.60x | 1.37x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 27.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 20.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 4.90x | 1.61x | 1.26x | 2.20x | 2.24x | 10/10 |
| corpus_materialize | 1.00x | 1.46x | 1.17x | 1.08x | 1.87x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.04x | 1.71x | 2.13x | 1.78x | 2.09x | 1.40x | 10/10 |
| replay_trusted_native | 1.01x | 0.84x | 1.63x | 2.08x | 2.67x | 3.03x | 9/10 |
| replay_checked_native | 1.13x | 2.45x | 1.11x | 8.39x | 8.27x | 9.76x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
