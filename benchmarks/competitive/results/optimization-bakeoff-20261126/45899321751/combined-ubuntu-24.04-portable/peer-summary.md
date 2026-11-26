| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 2.79x | 1.59x | 1.38x | 1.87x | 1.32x | 29/30 |
| materialize | 0.99x | 2.27x | 1.90x | 1.85x | 2.90x | 2.07x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.02x | 1.60x | 1.21x | 2.03x | 1.79x | 30/30 |
| perft_full_d3 | 0.99x | 1.34x | 1.63x | 1.39x | 1.91x | 1.58x | 30/30 |
| fen_parse | 1.01x | 1.35x | 1.34x | 1.07x | 1.37x | 1.09x | 30/30 |
| fen_write_native_ep | 1.00x | 3.08x | 10.11x | 1.87x | 28.79x | 2.47x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 25/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.36x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 9.02x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.19x | 1.49x | 4.22x | 1.35x | 1.58x | 30/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.44x | 57.61x | 109.91x | 122.50x | 99.85x | 5/5 |
| chess960_materialize | 1.01x | 1.84x | 1.78x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.06x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.08x | 42.73x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.07x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.01x | 21.55x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.61x | 2.29x | 1.27x | 2.31x | 3.16x | 10/10 |
| corpus_materialize | 0.99x | 1.49x | 1.39x | 1.13x | 1.80x | 1.23x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.96x | 3.03x | 2.22x | 2.96x | 1.56x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.67x | 1.61x | 2.21x | 2.03x | 9/10 |
| replay_checked_native | 1.00x | 2.04x | 1.16x | 8.91x | 8.74x | 9.92x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
