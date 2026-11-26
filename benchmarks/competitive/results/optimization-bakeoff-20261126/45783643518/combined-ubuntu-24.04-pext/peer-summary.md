| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.24x | 1.77x | 1.54x | 2.42x | 1.80x | 30/30 |
| materialize | 1.00x | 2.20x | 1.63x | 1.70x | 2.61x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 4.04x | 1.60x | 1.26x | 2.50x | 1.97x | 30/30 |
| perft_full_d3 | 1.01x | 1.47x | 1.69x | 1.64x | 2.18x | 1.83x | 30/30 |
| fen_parse | 1.08x | 1.40x | 1.38x | 1.16x | 1.59x | 1.14x | 30/30 |
| fen_write_native_ep | 0.96x | 3.06x | 6.50x | 1.17x | 18.43x | 1.55x | 30/30 |
| snapshot | 1.29x | 1.99x | 1.00x | 4.65x | 1.00x | 1.33x | 22/30 |
| san_parse_all | 1.01x | 1.94x | N/A | 4.33x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.02x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 1.05x | 1.75x | 1.11x | 3.34x | 1.17x | 1.34x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.13x | N/A | N/A | 6/6 |
| startpos | 1.10x | 1.01x | 78.85x | 148.52x | 187.48x | 143.09x | 5/5 |
| chess960_materialize | 0.94x | 1.57x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.41x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 38.10x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.72x | 16.47x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.77x | 2.53x | 1.40x | 3.12x | 2.78x | 10/10 |
| corpus_materialize | 1.00x | 1.46x | 1.34x | 1.11x | 1.76x | 1.19x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.32x | 3.15x | 2.29x | 3.05x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.74x | 1.26x | 1.29x | 1.83x | 2.05x | 8/10 |
| replay_checked_native | 1.00x | 2.06x | 1.03x | 8.47x | 8.19x | 9.52x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
