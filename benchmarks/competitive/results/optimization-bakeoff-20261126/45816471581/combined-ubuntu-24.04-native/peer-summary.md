| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.70x | 1.53x | 1.42x | 1.96x | 1.68x | 30/30 |
| materialize | 1.00x | 1.96x | 1.53x | 1.32x | 2.20x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.19x | 1.30x | 1.08x | 1.77x | 1.54x | 29/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.47x | 1.31x | 1.75x | 1.72x | 30/30 |
| fen_parse | 1.09x | 1.32x | 1.61x | 1.25x | 1.76x | 1.29x | 30/30 |
| fen_write_native_ep | 1.02x | 5.00x | 9.85x | 1.84x | 28.30x | 2.58x | 30/30 |
| snapshot | 1.00x | 2.09x | 1.23x | 4.33x | 2.20x | 1.33x | 30/30 |
| san_parse_all | 0.96x | 1.89x | N/A | 4.21x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.23x | N/A | 2.43x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.92x | 1.14x | 4.15x | 1.16x | 1.37x | 27/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 101.24x | 194.26x | 258.72x | 190.73x | 5/5 |
| chess960_materialize | 0.98x | 1.58x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.33x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 29.88x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.11x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 15.22x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.98x | 1.55x | 0.90x | 1.80x | 1.74x | 8/10 |
| corpus_materialize | 0.99x | 1.40x | 1.36x | 1.04x | 1.64x | 1.17x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.18x | 2.78x | 2.17x | 2.64x | 1.50x | 10/10 |
| replay_trusted_native | 0.99x | 0.75x | 1.44x | 1.61x | 1.97x | 2.14x | 8/10 |
| replay_checked_native | 0.97x | 2.15x | 1.05x | 7.99x | 7.40x | 9.04x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
