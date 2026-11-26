| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.11x | 4.31x | 1.36x | 1.43x | 1.85x | 1.65x | 30/30 |
| materialize | 1.10x | 1.91x | 1.36x | 1.00x | 2.48x | 1.29x | 27/30 |
| perft_bulk_d4 | 1.12x | 4.16x | 1.56x | 1.26x | 1.93x | 1.88x | 30/30 |
| perft_full_d3 | 1.81x | 1.42x | 1.59x | 1.24x | 1.83x | 1.86x | 30/30 |
| fen_parse | 0.98x | 1.20x | 1.25x | 1.26x | 1.57x | 1.29x | 30/30 |
| fen_write_native_ep | 1.06x | 3.02x | 6.44x | 1.25x | 20.07x | 1.62x | 30/30 |
| snapshot | 1.17x | 1.68x | 1.11x | 2.36x | 1.10x | 1.01x | 27/30 |
| san_parse_all | 1.11x | 1.60x | N/A | 3.37x | N/A | N/A | 12/12 |
| san_render_native | 1.18x | 11.54x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.25x | 0.85x | 0.86x | 4.19x | 0.84x | 0.95x | 9/30 |
| make_unmake_full | 1.23x | N/A | N/A | 1.01x | N/A | N/A | 4/6 |
| startpos | 1.03x | 1.23x | 69.40x | 137.86x | 163.22x | 132.37x | 5/5 |
| chess960_materialize | 1.00x | 1.53x | 1.21x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.90x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 40.19x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.94x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.16x | 19.51x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.96x | 6.31x | 2.22x | 1.50x | 2.90x | 2.90x | 10/10 |
| corpus_materialize | 0.93x | 1.30x | 1.22x | 1.00x | 1.69x | 1.06x | 8/10 |
| corpus_uci_sorted | 0.99x | 1.24x | 1.77x | 1.38x | 1.80x | 0.93x | 9/10 |
| replay_trusted_native | 0.96x | 0.44x | 1.16x | 1.13x | 1.20x | 1.23x | 7/10 |
| replay_checked_native | 0.95x | 1.86x | 0.84x | 7.04x | 6.12x | 7.48x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
