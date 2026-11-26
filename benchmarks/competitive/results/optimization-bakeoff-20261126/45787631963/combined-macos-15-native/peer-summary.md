| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.09x | 4.42x | 1.42x | 1.52x | 1.98x | 1.81x | 29/30 |
| materialize | 1.14x | 2.02x | 1.51x | 1.13x | 2.27x | 1.34x | 28/30 |
| perft_bulk_d4 | 0.92x | 3.77x | 1.39x | 1.19x | 1.77x | 1.70x | 29/30 |
| perft_full_d3 | 1.10x | 1.63x | 1.80x | 1.37x | 1.90x | 2.11x | 30/30 |
| fen_parse | 1.06x | 1.23x | 1.36x | 1.41x | 1.80x | 1.42x | 30/30 |
| fen_write_native_ep | 1.05x | 4.40x | 9.47x | 1.81x | 29.58x | 2.23x | 30/30 |
| snapshot | 0.99x | 1.78x | 0.99x | 2.22x | 1.18x | 0.98x | 20/30 |
| san_parse_all | 1.12x | 1.92x | N/A | 4.29x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.17x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 1.04x | 1.09x | 5.29x | 1.04x | 1.23x | 23/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.37x | 1.22x | 71.22x | 136.11x | 181.40x | 151.51x | 5/5 |
| chess960_materialize | 1.06x | 1.60x | 1.38x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.82x | 2.31x | 1.26x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 42.42x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.86x | 19.27x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.63x | 4.22x | 1.38x | 0.84x | 1.68x | 1.72x | 8/10 |
| corpus_materialize | 1.22x | 1.40x | 1.32x | 1.19x | 1.84x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.12x | 1.83x | 2.59x | 2.19x | 2.48x | 1.52x | 10/10 |
| replay_trusted_native | 0.95x | 0.54x | 1.36x | 1.49x | 1.57x | 1.59x | 8/10 |
| replay_checked_native | 1.12x | 2.18x | 0.94x | 7.25x | 6.28x | 8.17x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
