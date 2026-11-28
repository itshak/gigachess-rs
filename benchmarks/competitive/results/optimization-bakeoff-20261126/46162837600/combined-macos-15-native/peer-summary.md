| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.06x | 5.21x | 1.83x | 1.85x | 2.39x | 2.23x | 30/30 |
| materialize | 0.94x | 1.79x | 1.35x | 1.01x | 2.11x | 1.29x | 28/30 |
| perft_bulk_d4 | 1.07x | 4.44x | 1.59x | 1.24x | 1.98x | 1.90x | 30/30 |
| perft_full_d3 | 1.09x | 1.68x | 1.87x | 1.44x | 2.13x | 2.16x | 30/30 |
| fen_parse | 1.03x | 1.42x | 1.46x | 1.34x | 1.72x | 1.41x | 30/30 |
| fen_write_native_ep | 0.94x | 4.13x | 8.78x | 1.66x | 27.27x | 2.11x | 30/30 |
| snapshot | 1.08x | 1.74x | 1.12x | 2.48x | 1.16x | 1.11x | 27/30 |
| san_parse_all | 1.02x | 1.71x | N/A | 3.68x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 12.63x | N/A | 3.01x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 0.99x | 1.05x | 4.99x | 1.02x | 1.16x | 18/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.14x | 1.21x | 68.78x | 139.33x | 160.69x | 133.74x | 5/5 |
| chess960_materialize | 0.75x | 1.75x | 1.19x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 3.35x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.12x | 38.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.69x | 1.54x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.34x | 19.60x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.27x | 6.99x | 2.39x | 1.33x | 2.58x | 2.80x | 10/10 |
| corpus_materialize | 1.12x | 1.50x | 1.35x | 1.16x | 1.95x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.16x | 2.04x | 2.94x | 2.02x | 2.63x | 1.73x | 10/10 |
| replay_trusted_native | 1.02x | 0.72x | 1.77x | 1.80x | 2.00x | 2.07x | 8/10 |
| replay_checked_native | 1.07x | 2.65x | 1.26x | 9.64x | 8.35x | 10.44x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
