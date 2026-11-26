| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 4.83x | 1.67x | 1.80x | 2.42x | 1.97x | 30/30 |
| materialize | 1.23x | 2.09x | 1.54x | 1.05x | 2.26x | 1.45x | 28/30 |
| perft_bulk_d4 | 1.12x | 4.08x | 1.50x | 1.23x | 1.87x | 1.93x | 29/30 |
| perft_full_d3 | 1.16x | 1.52x | 1.69x | 1.34x | 1.94x | 2.09x | 30/30 |
| fen_parse | 1.06x | 1.22x | 1.58x | 1.33x | 1.92x | 1.41x | 30/30 |
| fen_write_native_ep | 1.13x | 4.17x | 13.61x | 1.68x | 26.19x | 2.11x | 30/30 |
| snapshot | 0.93x | 1.58x | 1.04x | 2.29x | 1.05x | 0.89x | 22/30 |
| san_parse_all | 1.12x | 1.70x | N/A | 3.90x | N/A | N/A | 12/12 |
| san_render_native | 1.11x | 12.08x | N/A | 2.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.23x | 1.26x | 6.06x | 1.18x | 1.30x | 29/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.86x | 1.69x | 97.86x | 187.56x | 265.18x | 199.75x | 5/5 |
| chess960_materialize | 0.72x | 1.68x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.78x | 3.31x | 1.43x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.79x | 38.32x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.69x | 0.86x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.79x | 22.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 6.09x | 1.99x | 1.32x | 2.27x | 2.49x | 9/10 |
| corpus_materialize | 0.86x | 1.57x | 1.59x | 1.17x | 2.05x | 1.36x | 10/10 |
| corpus_uci_sorted | 0.94x | 2.03x | 2.64x | 1.87x | 2.64x | 1.35x | 10/10 |
| replay_trusted_native | 0.87x | 0.67x | 1.70x | 1.69x | 1.84x | 1.84x | 8/10 |
| replay_checked_native | 0.99x | 2.68x | 1.28x | 10.62x | 8.29x | 11.10x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
