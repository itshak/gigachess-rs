| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.31x | 1.62x | 1.67x | 2.05x | 2.01x | 30/30 |
| materialize | 0.92x | 1.91x | 1.42x | 0.98x | 2.10x | 1.30x | 26/30 |
| perft_bulk_d4 | 1.04x | 4.31x | 1.57x | 1.35x | 2.01x | 2.04x | 30/30 |
| perft_full_d3 | 0.98x | 1.64x | 1.81x | 1.39x | 1.92x | 1.89x | 30/30 |
| fen_parse | 1.04x | 1.24x | 1.43x | 1.37x | 1.73x | 1.41x | 30/30 |
| fen_write_native_ep | 1.02x | 4.26x | 8.77x | 1.79x | 29.04x | 2.31x | 30/30 |
| snapshot | 1.02x | 1.45x | 1.06x | 2.20x | 1.09x | 0.87x | 21/30 |
| san_parse_all | 0.98x | 1.91x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 0.93x | 11.72x | N/A | 2.71x | N/A | N/A | 12/12 |
| apply_children_full | 0.96x | 0.99x | 1.10x | 4.84x | 0.99x | 1.05x | 20/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.21x | N/A | N/A | 5/6 |
| startpos | 0.98x | 1.13x | 74.39x | 131.05x | 166.12x | 134.73x | 5/5 |
| chess960_materialize | 0.98x | 1.54x | 1.27x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.87x | 3.13x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 35.52x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.08x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 18.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.45x | 2.23x | 1.29x | 2.53x | 2.74x | 10/10 |
| corpus_materialize | 1.06x | 1.56x | 1.54x | 1.35x | 2.08x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.02x | 1.91x | 2.81x | 1.81x | 2.39x | 1.29x | 10/10 |
| replay_trusted_native | 0.93x | 0.54x | 1.40x | 1.46x | 1.63x | 1.61x | 8/10 |
| replay_checked_native | 0.95x | 2.17x | 1.01x | 6.94x | 6.66x | 8.48x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
