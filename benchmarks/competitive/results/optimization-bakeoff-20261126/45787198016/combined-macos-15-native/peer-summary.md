| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.15x | 4.26x | 1.55x | 1.42x | 1.97x | 1.90x | 29/30 |
| materialize | 1.06x | 1.82x | 1.43x | 0.95x | 2.11x | 1.17x | 24/30 |
| perft_bulk_d4 | 1.13x | 4.35x | 1.63x | 1.34x | 2.06x | 2.09x | 30/30 |
| perft_full_d3 | 1.12x | 1.53x | 1.71x | 1.28x | 1.85x | 1.91x | 30/30 |
| fen_parse | 0.98x | 1.07x | 1.15x | 1.19x | 1.46x | 1.30x | 28/30 |
| fen_write_native_ep | 1.18x | 3.96x | 8.54x | 1.78x | 26.54x | 2.27x | 30/30 |
| snapshot | 1.19x | 1.94x | 1.21x | 2.63x | 1.23x | 1.12x | 27/30 |
| san_parse_all | 1.12x | 1.73x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 1.25x | 11.86x | N/A | 2.74x | N/A | N/A | 12/12 |
| apply_children_full | 1.17x | 1.06x | 1.08x | 5.69x | 1.08x | 1.23x | 23/30 |
| make_unmake_full | 1.25x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 1.01x | 1.11x | 71.95x | 145.87x | 166.76x | 131.77x | 5/5 |
| chess960_materialize | 0.99x | 1.61x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.03x | 2.95x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 42.26x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.18x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.17x | 25.05x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.94x | 1.94x | 1.11x | 2.11x | 2.31x | 10/10 |
| corpus_materialize | 1.25x | 1.49x | 1.51x | 1.15x | 1.93x | 1.19x | 9/10 |
| corpus_uci_sorted | 1.18x | 1.82x | 2.66x | 2.05x | 2.63x | 1.54x | 10/10 |
| replay_trusted_native | 1.10x | 0.55x | 1.60x | 1.54x | 1.72x | 1.60x | 8/10 |
| replay_checked_native | 1.30x | 2.31x | 1.12x | 8.49x | 7.13x | 8.54x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
