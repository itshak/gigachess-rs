| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.92x | 1.68x | 1.42x | 2.16x | 1.72x | 30/30 |
| materialize | 0.99x | 2.10x | 1.67x | 1.68x | 2.57x | 1.78x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.23x | 1.33x | 1.01x | 1.96x | 1.58x | 27/30 |
| perft_full_d3 | 0.99x | 1.46x | 1.73x | 1.62x | 2.09x | 1.77x | 30/30 |
| fen_parse | 0.99x | 1.39x | 1.27x | 1.08x | 1.38x | 1.17x | 29/30 |
| fen_write_native_ep | 1.01x | 4.49x | 9.85x | 2.07x | 28.97x | 2.56x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 7.60x | 1.00x | 1.33x | 24/30 |
| san_parse_all | 1.00x | 1.93x | N/A | 4.44x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 9.21x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.09x | 1.29x | 3.94x | 1.31x | 1.62x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.25x | 103.79x | 179.37x | 219.55x | 197.84x | 5/5 |
| chess960_materialize | 1.01x | 1.63x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.32x | 1.41x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 41.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.92x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.83x | 16.43x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.26x | 1.63x | 0.90x | 1.94x | 1.78x | 8/10 |
| corpus_materialize | 1.01x | 1.35x | 1.27x | 1.05x | 1.65x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.31x | 2.94x | 2.26x | 2.86x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.88x | 1.63x | 1.62x | 2.20x | 2.61x | 9/10 |
| replay_checked_native | 1.00x | 1.92x | 1.03x | 8.26x | 7.84x | 9.36x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
