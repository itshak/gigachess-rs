| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.68x | 1.79x | 2.17x | 2.18x | 30/30 |
| materialize | 1.00x | 1.94x | 1.54x | 1.33x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.88x | 1.59x | 1.43x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.53x | 2.25x | 2.12x | 30/30 |
| fen_parse | 1.00x | 1.22x | 1.31x | 1.24x | 1.72x | 1.39x | 30/30 |
| fen_write_native_ep | 1.00x | 4.48x | 9.31x | 1.60x | 30.21x | 2.39x | 30/30 |
| snapshot | 1.01x | 1.15x | 1.01x | 1.49x | 0.99x | 0.93x | 20/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.27x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.85x | N/A | 2.53x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.26x | 1.48x | 4.08x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.28x | 76.62x | 128.21x | 177.56x | 143.53x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.99x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.86x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.53x | 1.70x | 1.36x | 2.01x | 2.18x | 10/10 |
| corpus_materialize | 1.01x | 1.63x | 1.52x | 1.21x | 1.90x | 1.38x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.94x | 2.07x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.00x | 0.63x | 1.64x | 1.98x | 2.23x | 2.25x | 8/10 |
| replay_checked_native | 1.00x | 2.17x | 1.05x | 7.03x | 6.26x | 8.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
