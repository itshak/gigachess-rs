| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.65x | 1.75x | 2.03x | 2.29x | 2.15x | 30/30 |
| materialize | 1.01x | 2.00x | 1.48x | 1.14x | 2.25x | 1.46x | 28/30 |
| perft_bulk_d4 | 1.11x | 4.58x | 1.65x | 1.43x | 2.21x | 2.24x | 30/30 |
| perft_full_d3 | 0.95x | 1.62x | 1.75x | 1.27x | 1.98x | 2.00x | 30/30 |
| fen_parse | 1.08x | 1.29x | 1.39x | 1.44x | 1.79x | 1.53x | 30/30 |
| fen_write_native_ep | 1.00x | 4.41x | 8.61x | 1.79x | 28.32x | 2.28x | 30/30 |
| snapshot | 0.75x | 1.32x | 0.93x | 1.96x | 0.90x | 0.71x | 15/30 |
| san_parse_all | 1.04x | 1.92x | N/A | 4.06x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 12.18x | N/A | 2.87x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 1.22x | 1.29x | 5.59x | 1.19x | 1.26x | 28/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.11x | 1.92x | 87.76x | 177.02x | 173.31x | 179.35x | 5/5 |
| chess960_materialize | 0.82x | 1.76x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.20x | 3.38x | 1.71x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 42.58x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.81x | 0.87x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.04x | 22.11x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.16x | 6.39x | 2.37x | 1.28x | 2.45x | 2.58x | 10/10 |
| corpus_materialize | 0.98x | 1.72x | 1.52x | 1.17x | 2.04x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.80x | 2.56x | 1.90x | 2.31x | 1.60x | 10/10 |
| replay_trusted_native | 0.96x | 0.61x | 1.46x | 1.76x | 1.63x | 1.79x | 8/10 |
| replay_checked_native | 1.02x | 2.60x | 1.22x | 9.58x | 8.01x | 10.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
