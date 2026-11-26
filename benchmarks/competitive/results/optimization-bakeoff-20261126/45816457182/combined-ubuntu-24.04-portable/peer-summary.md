| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 2.91x | 1.51x | 1.30x | 1.83x | 1.30x | 29/30 |
| materialize | 0.99x | 2.23x | 1.77x | 1.28x | 2.53x | 1.60x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.92x | 1.49x | 1.13x | 1.83x | 1.68x | 29/30 |
| perft_full_d3 | 0.97x | 1.40x | 1.71x | 1.29x | 1.92x | 1.74x | 30/30 |
| fen_parse | 1.10x | 1.71x | 1.65x | 1.37x | 2.10x | 1.39x | 30/30 |
| fen_write_native_ep | 0.96x | 2.62x | 8.39x | 1.48x | 24.32x | 1.92x | 30/30 |
| snapshot | 0.99x | 1.62x | 1.16x | 2.64x | 1.16x | 1.00x | 29/30 |
| san_parse_all | 1.01x | 1.77x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.95x | N/A | 2.34x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.05x | 1.39x | 4.90x | 1.19x | 1.25x | 30/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.28x | 59.59x | 83.33x | 115.27x | 81.16x | 5/5 |
| chess960_materialize | 1.05x | 1.75x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.11x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 32.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.57x | 20.03x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.03x | 1.67x | 1.02x | 1.79x | 2.67x | 9/10 |
| corpus_materialize | 0.98x | 1.65x | 1.56x | 1.13x | 2.15x | 1.34x | 10/10 |
| corpus_uci_sorted | 1.05x | 2.02x | 2.81x | 2.25x | 2.83x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.64x | 1.41x | 1.44x | 1.94x | 1.76x | 8/10 |
| replay_checked_native | 0.97x | 1.87x | 1.07x | 7.32x | 7.20x | 8.48x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
