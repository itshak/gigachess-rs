| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.75x | 1.55x | 1.47x | 2.20x | 1.91x | 30/30 |
| materialize | 0.94x | 2.00x | 1.30x | 1.13x | 2.25x | 1.44x | 30/30 |
| perft_bulk_d4 | 0.92x | 3.87x | 1.39x | 1.21x | 2.21x | 1.96x | 30/30 |
| perft_full_d3 | 0.78x | 1.10x | 1.17x | 0.98x | 1.56x | 1.57x | 26/30 |
| fen_parse | 0.98x | 1.36x | 1.53x | 1.61x | 2.17x | 1.58x | 30/30 |
| fen_write_native_ep | 0.94x | 3.47x | 6.80x | 1.39x | 28.98x | 1.88x | 30/30 |
| snapshot | 0.80x | 2.18x | 1.33x | 3.83x | 1.42x | 1.20x | 30/30 |
| san_parse_all | 1.00x | 1.90x | N/A | 4.66x | N/A | N/A | 12/12 |
| san_render_native | 0.86x | 9.79x | N/A | 3.37x | N/A | N/A | 12/12 |
| apply_children_full | 0.67x | 1.33x | 0.92x | 6.05x | 1.11x | 1.32x | 24/30 |
| make_unmake_full | 0.94x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 0.86x | 0.86x | 63.80x | 121.72x | 160.84x | 124.46x | 4/5 |
| chess960_materialize | 0.84x | 1.47x | 1.18x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.68x | 1.34x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 35.16x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.95x | 26.87x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.92x | 5.48x | 1.62x | 1.08x | 2.11x | 1.99x | 9/10 |
| corpus_materialize | 1.07x | 1.68x | 1.34x | 1.21x | 2.31x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.10x | 1.76x | 2.16x | 1.83x | 2.11x | 1.48x | 10/10 |
| replay_trusted_native | 1.10x | 0.91x | 1.56x | 1.85x | 2.75x | 3.61x | 9/10 |
| replay_checked_native | 0.90x | 2.32x | 1.09x | 8.00x | 8.16x | 10.06x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
