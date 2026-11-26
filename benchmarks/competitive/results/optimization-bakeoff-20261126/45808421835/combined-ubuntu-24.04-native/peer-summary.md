| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.18x | 1.53x | 1.47x | 2.30x | 1.78x | 30/30 |
| materialize | 1.01x | 1.96x | 1.58x | 1.13x | 2.02x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.92x | 1.53x | 1.25x | 2.40x | 2.08x | 30/30 |
| perft_full_d3 | 0.93x | 1.20x | 1.24x | 0.95x | 1.75x | 1.49x | 26/30 |
| fen_parse | 1.04x | 1.27x | 1.49x | 1.08x | 2.28x | 1.33x | 30/30 |
| fen_write_native_ep | 0.99x | 3.96x | 9.19x | 1.89x | 27.03x | 2.12x | 30/30 |
| snapshot | 0.80x | 1.69x | 1.21x | 2.85x | 1.21x | 1.10x | 30/30 |
| san_parse_all | 1.00x | 2.50x | N/A | 3.73x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 10.78x | N/A | 2.17x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.74x | 1.17x | 4.34x | 1.29x | 1.49x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.22x | N/A | N/A | 6/6 |
| startpos | 0.91x | 1.18x | 91.57x | 127.75x | 228.31x | 148.87x | 5/5 |
| chess960_materialize | 1.01x | 1.61x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.73x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.77x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 19.17x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.69x | 1.80x | 1.32x | 2.40x | 2.07x | 10/10 |
| corpus_materialize | 1.00x | 1.98x | 1.75x | 1.24x | 2.81x | 1.55x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.34x | 3.15x | 2.38x | 2.92x | 1.68x | 10/10 |
| replay_trusted_native | 1.12x | 0.91x | 1.52x | 1.78x | 3.38x | 4.39x | 9/10 |
| replay_checked_native | 1.02x | 2.55x | 0.96x | 5.53x | 6.34x | 7.77x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
