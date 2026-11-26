| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.19x | 1.68x | 1.48x | 2.06x | 1.43x | 30/30 |
| materialize | 1.07x | 2.48x | 1.87x | 1.37x | 2.50x | 1.72x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.12x | 1.63x | 1.28x | 2.03x | 1.75x | 30/30 |
| perft_full_d3 | 1.43x | 1.42x | 1.68x | 1.29x | 1.96x | 1.85x | 30/30 |
| fen_parse | 0.94x | 1.54x | 1.52x | 1.26x | 2.03x | 1.29x | 30/30 |
| fen_write_native_ep | 0.99x | 2.55x | 8.63x | 1.69x | 25.89x | 2.12x | 30/30 |
| snapshot | 1.00x | 1.95x | 1.65x | 2.85x | 1.13x | 0.98x | 24/30 |
| san_parse_all | 0.99x | 2.22x | N/A | 3.97x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.51x | N/A | 2.24x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.31x | 5.01x | 1.29x | 1.49x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.49x | 108.13x | 164.39x | 109.83x | 5/5 |
| chess960_materialize | 1.02x | 1.91x | 1.71x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.39x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 22.14x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 20.59x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.18x | 1.76x | 1.23x | 1.92x | 1.85x | 10/10 |
| corpus_materialize | 1.00x | 2.05x | 1.86x | 1.38x | 3.02x | 1.78x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.05x | 2.94x | 2.21x | 2.82x | 1.59x | 10/10 |
| replay_trusted_native | 1.06x | 0.98x | 1.96x | 2.27x | 3.18x | 2.82x | 8/10 |
| replay_checked_native | 1.00x | 2.42x | 1.07x | 5.82x | 6.23x | 7.77x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
