| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.17x | 5.04x | 1.63x | 1.73x | 2.27x | 2.05x | 30/30 |
| materialize | 1.06x | 2.05x | 1.57x | 1.19x | 2.41x | 1.45x | 30/30 |
| perft_bulk_d4 | 1.17x | 4.65x | 1.77x | 1.40x | 2.18x | 2.04x | 30/30 |
| perft_full_d3 | 1.07x | 1.59x | 1.77x | 1.38x | 1.97x | 2.06x | 30/30 |
| fen_parse | 1.09x | 1.31x | 1.41x | 1.41x | 1.73x | 1.46x | 30/30 |
| fen_write_native_ep | 1.00x | 4.20x | 9.05x | 1.74x | 27.79x | 2.23x | 30/30 |
| snapshot | 1.07x | 1.68x | 1.18x | 2.55x | 1.13x | 1.07x | 27/30 |
| san_parse_all | 1.09x | 1.94x | N/A | 3.96x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.22x | N/A | 2.54x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 0.97x | 1.05x | 4.97x | 1.00x | 1.24x | 19/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.14x | N/A | N/A | 5/6 |
| startpos | 1.01x | 1.21x | 70.58x | 140.26x | 160.54x | 132.21x | 5/5 |
| chess960_materialize | 1.10x | 1.66x | 1.52x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.27x | 3.53x | 1.65x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 40.78x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 1.01x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.44x | 2.28x | 1.32x | 2.67x | 2.90x | 10/10 |
| corpus_materialize | 1.13x | 1.46x | 1.39x | 1.14x | 1.88x | 1.24x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.94x | 2.86x | 2.02x | 2.79x | 1.53x | 10/10 |
| replay_trusted_native | 0.94x | 0.59x | 1.44x | 1.40x | 1.45x | 1.64x | 8/10 |
| replay_checked_native | 0.97x | 2.03x | 1.00x | 6.98x | 6.68x | 7.76x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
