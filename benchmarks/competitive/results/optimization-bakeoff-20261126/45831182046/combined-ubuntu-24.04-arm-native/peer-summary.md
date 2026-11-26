| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.05x | 1.54x | 1.66x | 2.01x | 2.00x | 30/30 |
| materialize | 1.01x | 1.87x | 1.49x | 1.28x | 2.09x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.04x | 3.60x | 1.46x | 1.32x | 1.86x | 1.91x | 30/30 |
| perft_full_d3 | 1.01x | 1.17x | 1.27x | 1.06x | 1.56x | 1.47x | 29/30 |
| fen_parse | 1.01x | 1.23x | 1.32x | 1.27x | 1.75x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.37x | 9.26x | 1.60x | 29.91x | 2.39x | 30/30 |
| snapshot | 1.02x | 1.14x | 1.01x | 1.49x | 1.00x | 0.94x | 22/30 |
| san_parse_all | 0.99x | 1.84x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 12.27x | N/A | 2.68x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.46x | 4.02x | 1.21x | 1.20x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.67x | 131.45x | 177.13x | 143.31x | 5/5 |
| chess960_materialize | 1.05x | 1.66x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.85x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.02x | 25.87x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.03x | 19.39x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.04x | 4.06x | 1.56x | 1.25x | 1.84x | 2.03x | 10/10 |
| corpus_materialize | 1.01x | 1.54x | 1.48x | 1.19x | 1.85x | 1.33x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.10x | 2.91x | 2.10x | 2.86x | 1.45x | 10/10 |
| replay_trusted_native | 1.03x | 0.62x | 1.70x | 1.99x | 2.22x | 2.26x | 8/10 |
| replay_checked_native | 1.09x | 2.36x | 1.14x | 7.79x | 6.89x | 8.83x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
