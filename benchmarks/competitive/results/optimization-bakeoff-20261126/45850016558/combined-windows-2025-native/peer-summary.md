| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.91x | 1.69x | 1.64x | 2.53x | 2.12x | 30/30 |
| materialize | 0.92x | 1.90x | 1.31x | 1.19x | 2.19x | 1.47x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.11x | 1.63x | 1.41x | 2.34x | 2.07x | 30/30 |
| perft_full_d3 | 0.98x | 1.38x | 1.47x | 1.29x | 1.90x | 1.84x | 30/30 |
| fen_parse | 0.98x | 1.25x | 1.43x | 1.37x | 2.21x | 1.41x | 30/30 |
| fen_write_native_ep | 0.99x | 3.97x | 7.81x | 1.59x | 30.50x | 1.99x | 30/30 |
| snapshot | 1.00x | 2.20x | 1.01x | 3.41x | 1.02x | 1.00x | 24/30 |
| san_parse_all | 1.00x | 1.98x | N/A | 5.38x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.63x | N/A | 3.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.22x | 1.34x | 9.45x | 1.41x | 1.53x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 0.86x | 1.07x | 68.53x | 141.60x | 208.68x | 147.08x | 5/5 |
| chess960_materialize | 1.03x | 1.63x | 1.35x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 3.06x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.13x | 29.47x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 21.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.44x | 1.84x | 1.37x | 2.46x | 2.45x | 10/10 |
| corpus_materialize | 1.07x | 1.61x | 1.31x | 1.15x | 2.02x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.80x | 2.22x | 1.76x | 2.10x | 1.39x | 10/10 |
| replay_trusted_native | 1.04x | 0.87x | 1.62x | 2.13x | 2.68x | 3.07x | 9/10 |
| replay_checked_native | 1.03x | 2.27x | 1.01x | 7.71x | 7.63x | 9.16x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
