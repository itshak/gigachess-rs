| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.39x | 1.62x | 1.57x | 2.42x | 1.97x | 30/30 |
| materialize | 1.01x | 1.91x | 1.29x | 1.10x | 2.21x | 1.43x | 29/30 |
| perft_bulk_d4 | 1.02x | 3.69x | 1.57x | 1.33x | 2.27x | 2.07x | 30/30 |
| perft_full_d3 | 1.07x | 1.28x | 1.58x | 1.07x | 1.81x | 1.61x | 28/30 |
| fen_parse | 1.04x | 1.52x | 1.58x | 1.54x | 2.58x | 1.65x | 30/30 |
| fen_write_native_ep | 1.01x | 3.97x | 7.82x | 1.65x | 31.65x | 2.03x | 30/30 |
| snapshot | 1.25x | 1.80x | 1.26x | 4.16x | 1.33x | 1.20x | 30/30 |
| san_parse_all | 1.04x | 1.98x | N/A | 4.82x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.50x | N/A | 3.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.66x | 1.12x | 7.04x | 1.32x | 1.49x | 28/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 1.20x | 1.20x | 101.07x | 163.52x | 253.13x | 179.19x | 5/5 |
| chess960_materialize | 1.11x | 1.58x | 1.28x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.10x | 2.81x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.94x | 15.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.93x | 4.15x | 1.67x | 1.23x | 2.22x | 2.02x | 9/10 |
| corpus_materialize | 0.99x | 1.72x | 1.32x | 1.14x | 2.73x | 1.49x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.83x | 2.34x | 1.94x | 2.22x | 1.45x | 10/10 |
| replay_trusted_native | 0.99x | 0.78x | 1.34x | 1.67x | 2.69x | 3.33x | 9/10 |
| replay_checked_native | 0.99x | 2.08x | 0.91x | 5.36x | 6.07x | 7.06x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
