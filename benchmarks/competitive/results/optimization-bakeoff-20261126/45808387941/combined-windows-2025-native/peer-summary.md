| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 4.11x | 1.47x | 1.45x | 2.06x | 1.80x | 30/30 |
| materialize | 1.00x | 1.86x | 1.27x | 1.16x | 2.15x | 1.54x | 30/30 |
| perft_bulk_d4 | 0.91x | 3.52x | 1.38x | 1.20x | 2.01x | 1.83x | 30/30 |
| perft_full_d3 | 1.00x | 1.32x | 1.39x | 1.25x | 1.84x | 1.77x | 30/30 |
| fen_parse | 0.92x | 1.16x | 1.30x | 1.26x | 2.01x | 1.32x | 30/30 |
| fen_write_native_ep | 1.00x | 4.09x | 8.15x | 1.57x | 31.94x | 2.00x | 30/30 |
| snapshot | 0.99x | 1.85x | 0.98x | 4.20x | 0.99x | 1.00x | 22/30 |
| san_parse_all | 1.00x | 2.06x | N/A | 5.67x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.89x | N/A | 3.84x | N/A | N/A | 12/12 |
| apply_children_full | 1.13x | 2.05x | 1.25x | 8.74x | 1.34x | 1.45x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.48x | 1.25x | 79.55x | 170.16x | 252.11x | 185.01x | 5/5 |
| chess960_materialize | 1.06x | 1.53x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.20x | 2.77x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 27.81x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.09x | 1.10x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 19.09x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.94x | 1.64x | 1.25x | 2.28x | 2.21x | 10/10 |
| corpus_materialize | 0.94x | 1.49x | 1.20x | 1.07x | 1.87x | 1.35x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.76x | 2.18x | 1.77x | 2.13x | 1.39x | 10/10 |
| replay_trusted_native | 1.04x | 0.74x | 1.38x | 1.82x | 2.26x | 2.61x | 8/10 |
| replay_checked_native | 1.01x | 2.25x | 0.98x | 7.63x | 7.56x | 9.01x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
