| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 5.10x | 1.81x | 1.76x | 2.55x | 2.20x | 30/30 |
| materialize | 1.04x | 2.03x | 1.40x | 1.29x | 2.38x | 1.61x | 30/30 |
| perft_bulk_d4 | 1.03x | 4.17x | 1.64x | 1.42x | 2.35x | 2.15x | 30/30 |
| perft_full_d3 | 1.30x | 1.74x | 1.92x | 1.64x | 2.40x | 2.28x | 30/30 |
| fen_parse | 0.83x | 1.10x | 1.26x | 1.13x | 1.94x | 1.34x | 30/30 |
| fen_write_native_ep | 1.04x | 4.32x | 8.39x | 1.71x | 31.54x | 2.05x | 30/30 |
| snapshot | 1.00x | 1.98x | 1.01x | 4.09x | 1.00x | 1.04x | 26/30 |
| san_parse_all | 1.02x | 2.10x | N/A | 5.49x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.12x | N/A | 3.98x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.31x | 1.39x | 9.45x | 1.47x | 1.66x | 30/30 |
| make_unmake_full | 1.03x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 79.59x | 151.36x | 234.57x | 234.11x | 5/5 |
| chess960_materialize | 0.95x | 1.56x | 1.29x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.14x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.87x | 23.61x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.27x | 1.30x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.92x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.09x | 5.88x | 1.93x | 1.48x | 2.67x | 2.63x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.29x | 1.18x | 2.01x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.75x | 2.17x | 1.84x | 2.19x | 1.39x | 10/10 |
| replay_trusted_native | 1.04x | 0.88x | 1.68x | 2.14x | 2.75x | 3.14x | 9/10 |
| replay_checked_native | 0.98x | 2.37x | 1.06x | 8.26x | 7.99x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
