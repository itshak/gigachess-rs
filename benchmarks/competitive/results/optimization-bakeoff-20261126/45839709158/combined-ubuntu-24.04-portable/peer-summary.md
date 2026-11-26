| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.96x | 1.65x | 1.43x | 2.15x | 1.37x | 30/30 |
| materialize | 1.09x | 2.34x | 1.84x | 1.86x | 2.92x | 2.10x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.00x | 1.59x | 1.19x | 2.13x | 1.78x | 29/30 |
| perft_full_d3 | 0.99x | 1.28x | 1.50x | 1.33x | 1.82x | 1.50x | 30/30 |
| fen_parse | 0.89x | 1.47x | 1.35x | 1.04x | 1.45x | 1.07x | 29/30 |
| fen_write_native_ep | 0.99x | 2.96x | 9.80x | 2.01x | 27.71x | 2.18x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.26x | 1.25x | 1.00x | 28/30 |
| san_parse_all | 1.09x | 2.11x | N/A | 4.34x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 9.13x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.14x | 1.45x | 4.36x | 1.31x | 1.54x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.85x | 99.56x | 135.36x | 98.58x | 5/5 |
| chess960_materialize | 1.01x | 1.90x | 1.82x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.13x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.93x | 38.35x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 22.01x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.26x | 2.11x | 1.17x | 2.14x | 2.92x | 10/10 |
| corpus_materialize | 1.01x | 1.51x | 1.42x | 1.15x | 1.85x | 1.25x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.98x | 2.97x | 2.30x | 2.92x | 1.61x | 10/10 |
| replay_trusted_native | 0.99x | 0.85x | 1.61x | 1.61x | 2.21x | 2.02x | 9/10 |
| replay_checked_native | 0.97x | 1.87x | 1.03x | 8.06x | 8.00x | 9.01x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
