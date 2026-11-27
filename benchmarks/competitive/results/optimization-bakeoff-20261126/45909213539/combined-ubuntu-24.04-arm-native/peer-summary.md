| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.39x | 1.67x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.33x | 2.15x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.57x | 1.42x | 2.01x | 2.06x | 30/30 |
| perft_full_d3 | 1.00x | 1.48x | 1.61x | 1.33x | 1.98x | 1.86x | 30/30 |
| fen_parse | 1.00x | 1.24x | 1.33x | 1.30x | 1.74x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 4.46x | 9.27x | 1.58x | 29.90x | 2.39x | 30/30 |
| snapshot | 1.00x | 1.05x | 1.09x | 1.45x | 1.09x | 0.96x | 24/30 |
| san_parse_all | 1.00x | 1.87x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.38x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.00x | 1.20x | 1.19x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 76.74x | 130.70x | 169.38x | 138.10x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.99x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.02x | 19.32x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.42x | 1.72x | 1.37x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.24x | 1.91x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.10x | 2.92x | 2.08x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.66x | 2.03x | 2.26x | 2.31x | 8/10 |
| replay_checked_native | 1.00x | 2.31x | 1.14x | 7.68x | 6.81x | 8.72x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
