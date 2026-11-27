| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 3.32x | 1.67x | 1.53x | 2.16x | 1.55x | 30/30 |
| materialize | 1.00x | 2.14x | 1.93x | 1.35x | 2.58x | 1.63x | 30/30 |
| perft_bulk_d4 | 1.09x | 3.31x | 1.77x | 1.38x | 2.23x | 1.92x | 30/30 |
| perft_full_d3 | 1.18x | 1.63x | 1.85x | 1.54x | 2.23x | 2.09x | 30/30 |
| fen_parse | 0.89x | 1.39x | 1.32x | 1.09x | 1.94x | 1.15x | 30/30 |
| fen_write_native_ep | 0.96x | 2.77x | 9.31x | 1.69x | 27.49x | 1.94x | 30/30 |
| snapshot | 1.00x | 1.68x | 1.23x | 3.04x | 1.06x | 1.02x | 30/30 |
| san_parse_all | 1.01x | 2.06x | N/A | 4.41x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 10.61x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.31x | 1.57x | 4.55x | 1.44x | 1.38x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 65.54x | 115.44x | 184.83x | 113.71x | 5/5 |
| chess960_materialize | 1.01x | 1.64x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.35x | 1.57x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 23.80x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 0.94x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.11x | 4.30x | 2.11x | 1.42x | 2.34x | 2.83x | 10/10 |
| corpus_materialize | 1.01x | 1.64x | 1.63x | 1.20x | 2.12x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.24x | 3.29x | 2.47x | 3.19x | 1.83x | 10/10 |
| replay_trusted_native | 0.99x | 0.91x | 1.76x | 2.00x | 2.64x | 2.51x | 9/10 |
| replay_checked_native | 1.00x | 2.31x | 1.15x | 7.98x | 8.07x | 9.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
