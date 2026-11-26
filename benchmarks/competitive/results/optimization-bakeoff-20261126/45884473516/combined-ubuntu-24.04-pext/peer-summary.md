| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.45x | 1.87x | 1.62x | 2.45x | 1.91x | 30/30 |
| materialize | 1.01x | 2.27x | 1.68x | 1.76x | 2.82x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.80x | 1.51x | 1.19x | 2.33x | 1.84x | 29/30 |
| perft_full_d3 | 1.01x | 1.43x | 1.55x | 1.54x | 1.95x | 1.90x | 30/30 |
| fen_parse | 1.00x | 1.28x | 1.31x | 1.16x | 1.45x | 1.25x | 30/30 |
| fen_write_native_ep | 0.97x | 4.86x | 9.92x | 1.97x | 28.49x | 2.09x | 30/30 |
| snapshot | 1.00x | 1.66x | 1.34x | 4.01x | 1.33x | 1.34x | 30/30 |
| san_parse_all | 0.99x | 1.86x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.11x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 2.07x | 1.31x | 3.96x | 1.23x | 1.69x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.13x | 1.25x | 99.21x | 183.54x | 215.57x | 195.71x | 5/5 |
| chess960_materialize | 1.07x | 1.80x | 1.70x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.56x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.04x | 38.74x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.81x | 18.92x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.94x | 5.12x | 1.90x | 1.04x | 2.29x | 2.13x | 9/10 |
| corpus_materialize | 1.00x | 1.42x | 1.32x | 1.07x | 1.74x | 1.17x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.29x | 3.06x | 2.23x | 3.02x | 1.56x | 10/10 |
| replay_trusted_native | 0.92x | 0.82x | 1.49x | 1.49x | 2.05x | 2.41x | 8/10 |
| replay_checked_native | 0.95x | 2.15x | 1.10x | 8.68x | 8.50x | 9.85x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
