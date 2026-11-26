| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.26x | 1.45x | 1.54x | 2.08x | 1.72x | 30/30 |
| materialize | 1.12x | 2.05x | 1.51x | 1.05x | 2.45x | 1.44x | 28/30 |
| perft_bulk_d4 | 0.82x | 3.63x | 1.37x | 1.23x | 1.78x | 1.91x | 30/30 |
| perft_full_d3 | 0.97x | 1.71x | 1.89x | 1.45x | 2.18x | 2.07x | 30/30 |
| fen_parse | 0.98x | 1.25x | 1.36x | 1.28x | 1.65x | 1.36x | 30/30 |
| fen_write_native_ep | 0.91x | 3.96x | 8.57x | 1.58x | 25.52x | 2.08x | 30/30 |
| snapshot | 1.04x | 1.80x | 1.13x | 2.45x | 1.17x | 1.00x | 25/30 |
| san_parse_all | 0.91x | 1.75x | N/A | 3.70x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 12.67x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.14x | 1.12x | 1.19x | 5.78x | 1.18x | 1.28x | 29/30 |
| make_unmake_full | 1.12x | N/A | N/A | 1.23x | N/A | N/A | 5/6 |
| startpos | 0.77x | 0.98x | 57.63x | 113.08x | 129.88x | 104.58x | 4/5 |
| chess960_materialize | 0.98x | 1.57x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 2.68x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 40.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.21x | 18.81x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 5.05x | 1.83x | 1.13x | 2.01x | 2.71x | 10/10 |
| corpus_materialize | 0.97x | 1.46x | 1.46x | 1.30x | 2.03x | 1.29x | 10/10 |
| corpus_uci_sorted | 0.97x | 1.77x | 2.67x | 1.89x | 2.41x | 1.29x | 10/10 |
| replay_trusted_native | 1.15x | 0.58x | 1.55x | 1.55x | 1.75x | 1.60x | 8/10 |
| replay_checked_native | 0.97x | 2.15x | 1.00x | 7.89x | 7.10x | 8.75x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
