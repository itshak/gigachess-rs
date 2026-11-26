| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.03x | 1.74x | 1.45x | 2.26x | 1.78x | 30/30 |
| materialize | 1.08x | 2.03x | 1.60x | 1.65x | 2.50x | 1.94x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.59x | 1.46x | 1.12x | 2.22x | 1.76x | 30/30 |
| perft_full_d3 | 1.17x | 1.48x | 1.76x | 1.66x | 2.24x | 1.84x | 30/30 |
| fen_parse | 1.04x | 1.28x | 1.28x | 1.00x | 1.39x | 1.08x | 26/30 |
| fen_write_native_ep | 0.95x | 2.83x | 6.11x | 1.32x | 17.63x | 1.63x | 30/30 |
| snapshot | 0.67x | 1.34x | 0.87x | 3.02x | 0.87x | 0.99x | 13/30 |
| san_parse_all | 0.99x | 1.89x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 8.74x | N/A | 2.05x | N/A | N/A | 12/12 |
| apply_children_full | 0.67x | 1.20x | 0.77x | 2.21x | 0.77x | 0.91x | 12/30 |
| make_unmake_full | 0.87x | N/A | N/A | 1.00x | N/A | N/A | 1/6 |
| startpos | 0.81x | 1.21x | 80.64x | 141.34x | 173.61x | 142.48x | 5/5 |
| chess960_materialize | 1.01x | 1.68x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 2.09x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 38.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 20.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.53x | 2.45x | 1.33x | 2.97x | 2.69x | 10/10 |
| corpus_materialize | 1.02x | 1.45x | 1.34x | 1.09x | 1.75x | 1.23x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.22x | 2.86x | 2.18x | 2.84x | 1.56x | 10/10 |
| replay_trusted_native | 0.97x | 0.70x | 1.30x | 1.23x | 1.75x | 2.05x | 8/10 |
| replay_checked_native | 0.99x | 1.95x | 1.05x | 8.29x | 8.27x | 9.68x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
