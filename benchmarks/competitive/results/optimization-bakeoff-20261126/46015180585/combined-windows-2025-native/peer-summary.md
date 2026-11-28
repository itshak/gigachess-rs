| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 5.36x | 1.79x | 1.65x | 2.51x | 2.16x | 30/30 |
| materialize | 1.02x | 2.12x | 1.39x | 1.24x | 2.43x | 1.56x | 30/30 |
| perft_bulk_d4 | 1.05x | 4.60x | 1.68x | 1.44x | 2.49x | 2.33x | 30/30 |
| perft_full_d3 | 1.31x | 1.83x | 2.16x | 1.66x | 2.57x | 2.65x | 30/30 |
| fen_parse | 1.08x | 1.38x | 1.54x | 1.58x | 2.14x | 1.66x | 30/30 |
| fen_write_native_ep | 1.01x | 3.77x | 7.44x | 1.54x | 31.99x | 2.11x | 30/30 |
| snapshot | 1.00x | 2.52x | 1.51x | 5.48x | 1.51x | 1.51x | 30/30 |
| san_parse_all | 0.97x | 1.92x | N/A | 4.68x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 11.12x | N/A | 4.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 1.87x | 1.41x | 8.99x | 1.42x | 2.00x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 0.97x | 1.57x | 78.25x | 140.77x | 184.73x | 142.09x | 5/5 |
| chess960_materialize | 0.96x | 1.72x | 1.39x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.97x | 2.98x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 35.07x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.52x | 1.65x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.98x | 28.71x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 6.42x | 1.85x | 1.24x | 2.43x | 2.36x | 9/10 |
| corpus_materialize | 1.00x | 1.63x | 1.28x | 1.09x | 2.20x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.62x | 1.96x | 1.80x | 1.89x | 1.38x | 10/10 |
| replay_trusted_native | 0.99x | 0.72x | 1.40x | 1.69x | 2.49x | 2.97x | 8/10 |
| replay_checked_native | 1.05x | 2.56x | 1.14x | 9.03x | 8.89x | 11.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
