| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 2.90x | 1.48x | 1.35x | 1.89x | 1.33x | 30/30 |
| materialize | 1.01x | 2.02x | 1.80x | 1.29x | 2.40x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.95x | 1.57x | 1.23x | 1.97x | 1.71x | 30/30 |
| perft_full_d3 | 1.01x | 1.45x | 1.65x | 1.35x | 1.99x | 1.81x | 30/30 |
| fen_parse | 1.08x | 1.51x | 1.56x | 1.24x | 2.36x | 1.38x | 30/30 |
| fen_write_native_ep | 1.01x | 2.86x | 9.78x | 1.71x | 28.14x | 2.02x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.20x | 3.03x | 1.00x | 1.07x | 28/30 |
| san_parse_all | 0.98x | 2.02x | N/A | 4.45x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.13x | N/A | 2.41x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 2.77x | 1.44x | 4.27x | 1.47x | 1.30x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.35x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 64.70x | 107.56x | 178.58x | 112.22x | 5/5 |
| chess960_materialize | 1.00x | 1.52x | 1.46x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.11x | 1.38x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.06x | 27.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.06x | 20.07x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 3.97x | 1.93x | 1.28x | 2.11x | 2.57x | 10/10 |
| corpus_materialize | 1.02x | 1.65x | 1.64x | 1.18x | 2.13x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.20x | 3.29x | 2.45x | 3.19x | 1.81x | 10/10 |
| replay_trusted_native | 1.08x | 0.76x | 1.53x | 1.72x | 2.24x | 2.13x | 8/10 |
| replay_checked_native | 1.00x | 2.14x | 1.03x | 7.12x | 7.29x | 8.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
