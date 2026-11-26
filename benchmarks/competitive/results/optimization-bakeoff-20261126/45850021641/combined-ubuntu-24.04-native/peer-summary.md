| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.31x | 1.83x | 1.55x | 2.39x | 1.87x | 30/30 |
| materialize | 0.92x | 2.10x | 1.65x | 1.69x | 2.52x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.75x | 1.54x | 1.18x | 2.30x | 1.84x | 29/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.60x | 1.50x | 2.00x | 1.69x | 30/30 |
| fen_parse | 0.99x | 1.38x | 1.35x | 1.04x | 1.46x | 1.24x | 28/30 |
| fen_write_native_ep | 0.95x | 4.72x | 9.54x | 1.80x | 27.06x | 2.54x | 30/30 |
| snapshot | 0.99x | 1.82x | 1.15x | 4.33x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.75x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.04x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.06x | 1.30x | 3.92x | 1.30x | 1.63x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.11x | 88.69x | 153.32x | 198.94x | 179.42x | 5/5 |
| chess960_materialize | 1.11x | 1.73x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.52x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 39.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 21.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.40x | 2.06x | 1.11x | 2.43x | 2.21x | 9/10 |
| corpus_materialize | 1.00x | 1.39x | 1.29x | 1.06x | 1.69x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.26x | 2.99x | 2.24x | 2.91x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.89x | 1.61x | 1.54x | 2.18x | 2.58x | 9/10 |
| replay_checked_native | 1.01x | 2.04x | 1.08x | 8.63x | 8.32x | 9.98x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
