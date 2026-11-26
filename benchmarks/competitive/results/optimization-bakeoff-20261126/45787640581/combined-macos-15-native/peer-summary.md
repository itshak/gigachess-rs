| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 4.60x | 1.37x | 1.72x | 1.97x | 1.86x | 30/30 |
| materialize | 1.01x | 1.92x | 1.55x | 0.98x | 2.20x | 1.24x | 26/30 |
| perft_bulk_d4 | 0.83x | 3.46x | 1.28x | 1.03x | 1.67x | 1.55x | 26/30 |
| perft_full_d3 | 0.99x | 1.67x | 1.90x | 1.45x | 2.09x | 2.06x | 30/30 |
| fen_parse | 0.92x | 1.22x | 1.32x | 1.28x | 1.63x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 4.62x | 8.99x | 1.74x | 27.57x | 2.29x | 30/30 |
| snapshot | 1.02x | 1.52x | 1.06x | 2.27x | 1.00x | 0.90x | 22/30 |
| san_parse_all | 0.99x | 1.96x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.52x | N/A | 2.65x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.05x | 1.11x | 5.28x | 1.01x | 1.12x | 22/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.23x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.70x | 75.87x | 190.66x | 203.40x | 182.54x | 5/5 |
| chess960_materialize | 0.85x | 1.38x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.79x | 2.60x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.94x | 42.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.96x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 20.68x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.62x | 5.35x | 1.51x | 1.03x | 1.77x | 2.01x | 10/10 |
| corpus_materialize | 0.96x | 1.42x | 1.34x | 1.04x | 1.73x | 1.06x | 8/10 |
| corpus_uci_sorted | 0.94x | 1.71x | 2.80x | 1.76x | 2.16x | 1.36x | 10/10 |
| replay_trusted_native | 1.10x | 0.66x | 1.66x | 1.83x | 1.97x | 1.94x | 8/10 |
| replay_checked_native | 0.93x | 2.26x | 1.03x | 7.90x | 6.83x | 9.00x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
