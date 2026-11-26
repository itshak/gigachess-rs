| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.32x | 1.55x | 1.52x | 2.36x | 1.89x | 30/30 |
| materialize | 0.96x | 1.83x | 1.24x | 1.03x | 2.16x | 1.38x | 28/30 |
| perft_bulk_d4 | 1.01x | 3.75x | 1.53x | 1.35x | 2.47x | 2.10x | 30/30 |
| perft_full_d3 | 0.97x | 1.20x | 1.27x | 1.05x | 1.68x | 1.49x | 27/30 |
| fen_parse | 0.83x | 1.26x | 1.32x | 1.24x | 2.03x | 1.35x | 30/30 |
| fen_write_native_ep | 1.00x | 4.04x | 7.57x | 1.58x | 31.21x | 1.99x | 30/30 |
| snapshot | 1.00x | 1.81x | 1.26x | 4.12x | 1.25x | 1.16x | 30/30 |
| san_parse_all | 1.03x | 1.99x | N/A | 4.78x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.80x | N/A | 3.48x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.65x | 1.10x | 7.18x | 1.31x | 1.46x | 29/30 |
| make_unmake_full | 1.04x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 100.95x | 158.05x | 253.27x | 174.39x | 5/5 |
| chess960_materialize | 1.07x | 1.52x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 2.68x | 1.59x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 15.49x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.07x | 1.61x | 1.18x | 2.13x | 1.95x | 9/10 |
| corpus_materialize | 1.00x | 1.70x | 1.31x | 1.13x | 2.66x | 1.44x | 10/10 |
| corpus_uci_sorted | 1.06x | 1.87x | 2.36x | 1.96x | 2.23x | 1.49x | 10/10 |
| replay_trusted_native | 1.00x | 0.83x | 1.48x | 1.82x | 3.12x | 4.16x | 9/10 |
| replay_checked_native | 1.03x | 2.12x | 0.95x | 5.35x | 6.63x | 7.91x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
