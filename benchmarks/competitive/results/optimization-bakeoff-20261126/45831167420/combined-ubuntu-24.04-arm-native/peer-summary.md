| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.12x | 4.40x | 1.67x | 1.80x | 2.17x | 2.16x | 30/30 |
| materialize | 1.05x | 1.95x | 1.59x | 1.35x | 2.16x | 1.48x | 30/30 |
| perft_bulk_d4 | 1.12x | 3.86x | 1.57x | 1.43x | 2.01x | 2.05x | 30/30 |
| perft_full_d3 | 1.01x | 1.18x | 1.27x | 1.06x | 1.57x | 1.47x | 29/30 |
| fen_parse | 1.00x | 1.23x | 1.31x | 1.27x | 1.72x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.55x | 9.48x | 1.59x | 30.32x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.10x | 1.06x | 1.47x | 1.06x | 0.94x | 24/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.68x | N/A | 2.51x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.24x | 1.47x | 4.00x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.54x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.55x | 131.42x | 171.91x | 138.86x | 5/5 |
| chess960_materialize | 1.08x | 1.72x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.12x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 25.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 19.44x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.15x | 4.47x | 1.78x | 1.38x | 2.03x | 2.20x | 10/10 |
| corpus_materialize | 1.05x | 1.61x | 1.56x | 1.23x | 1.92x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.12x | 2.95x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 1.02x | 0.63x | 1.64x | 1.98x | 2.22x | 2.26x | 8/10 |
| replay_checked_native | 0.99x | 2.16x | 1.05x | 7.13x | 6.38x | 8.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
