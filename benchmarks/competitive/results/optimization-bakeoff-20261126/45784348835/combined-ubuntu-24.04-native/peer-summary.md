| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.00x | 1.71x | 1.45x | 2.25x | 1.75x | 30/30 |
| materialize | 1.00x | 2.10x | 1.58x | 1.64x | 2.61x | 1.88x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.96x | 1.77x | 1.21x | 2.41x | 1.92x | 30/30 |
| perft_full_d3 | 1.01x | 2.00x | 2.30x | 1.69x | 2.32x | 1.88x | 30/30 |
| fen_parse | 1.15x | 1.40x | 1.39x | 1.11x | 1.52x | 1.19x | 30/30 |
| fen_write_native_ep | 0.99x | 4.62x | 10.12x | 2.11x | 29.26x | 2.66x | 30/30 |
| snapshot | 0.71x | 1.73x | 1.08x | 3.05x | 1.08x | 1.15x | 30/30 |
| san_parse_all | 1.00x | 1.89x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.15x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.09x | 1.96x | 1.24x | 3.59x | 1.20x | 1.50x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 0.81x | 1.15x | 72.00x | 124.33x | 153.59x | 125.83x | 5/5 |
| chess960_materialize | 1.05x | 1.68x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.28x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 39.02x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.05x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.22x | 22.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.46x | 2.43x | 1.34x | 2.94x | 2.67x | 10/10 |
| corpus_materialize | 1.01x | 1.43x | 1.34x | 1.07x | 1.76x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.30x | 3.00x | 2.23x | 2.95x | 1.57x | 10/10 |
| replay_trusted_native | 1.14x | 0.82x | 1.47x | 1.46x | 2.06x | 2.41x | 9/10 |
| replay_checked_native | 1.01x | 2.01x | 1.12x | 8.48x | 8.28x | 9.97x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
