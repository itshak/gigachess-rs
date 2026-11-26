| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.95x | 4.06x | 1.44x | 1.38x | 2.00x | 1.74x | 30/30 |
| materialize | 0.97x | 1.86x | 1.28x | 1.21x | 2.13x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.61x | 1.43x | 1.26x | 2.04x | 1.81x | 30/30 |
| perft_full_d3 | 0.90x | 1.16x | 1.24x | 1.11x | 1.64x | 1.58x | 29/30 |
| fen_parse | 0.89x | 1.13x | 1.32x | 1.16x | 1.99x | 1.40x | 30/30 |
| fen_write_native_ep | 0.99x | 4.34x | 8.07x | 1.59x | 31.57x | 1.96x | 30/30 |
| snapshot | 1.08x | 1.70x | 1.18x | 2.78x | 1.16x | 1.02x | 29/30 |
| san_parse_all | 0.92x | 1.75x | N/A | 4.86x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 8.79x | N/A | 3.04x | N/A | N/A | 12/12 |
| apply_children_full | 0.85x | 1.61x | 0.99x | 6.57x | 1.04x | 1.19x | 24/30 |
| make_unmake_full | 0.82x | N/A | N/A | 1.01x | N/A | N/A | 3/6 |
| startpos | 1.48x | 1.29x | 79.59x | 151.35x | 238.00x | 180.16x | 5/5 |
| chess960_materialize | 0.93x | 1.40x | 1.16x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.89x | 2.25x | 1.19x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.05x | 28.71x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.21x | 23.62x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.28x | 1.78x | 1.34x | 2.45x | 2.34x | 10/10 |
| corpus_materialize | 1.00x | 1.57x | 1.27x | 1.16x | 1.96x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.71x | 2.15x | 1.77x | 2.10x | 1.37x | 10/10 |
| replay_trusted_native | 0.67x | 0.45x | 0.86x | 1.10x | 1.41x | 1.67x | 6/10 |
| replay_checked_native | 0.73x | 1.52x | 0.68x | 5.22x | 5.08x | 6.12x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
