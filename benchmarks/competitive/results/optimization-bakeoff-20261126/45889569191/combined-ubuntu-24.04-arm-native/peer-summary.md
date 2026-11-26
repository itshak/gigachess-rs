| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.47x | 1.70x | 1.82x | 2.21x | 2.21x | 30/30 |
| materialize | 1.00x | 1.96x | 1.56x | 1.36x | 2.17x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.01x | 3.91x | 1.59x | 1.45x | 2.04x | 2.09x | 30/30 |
| perft_full_d3 | 1.14x | 1.69x | 1.83x | 1.53x | 2.24x | 2.12x | 30/30 |
| fen_parse | 0.99x | 1.22x | 1.30x | 1.27x | 1.72x | 1.37x | 30/30 |
| fen_write_native_ep | 1.00x | 4.50x | 9.34x | 1.59x | 30.02x | 2.40x | 30/30 |
| snapshot | 1.01x | 1.10x | 1.05x | 1.47x | 1.04x | 0.95x | 23/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 12.48x | N/A | 2.69x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.05x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.55x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.31x | 76.40x | 129.26x | 169.39x | 135.09x | 5/5 |
| chess960_materialize | 1.00x | 1.71x | 1.58x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.95x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.45x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.08x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.49x | 1.71x | 1.39x | 2.06x | 2.23x | 10/10 |
| corpus_materialize | 1.00x | 1.62x | 1.53x | 1.25x | 1.94x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.11x | 2.94x | 2.10x | 2.86x | 1.46x | 10/10 |
| replay_trusted_native | 1.01x | 0.63x | 1.68x | 2.02x | 2.26x | 2.31x | 8/10 |
| replay_checked_native | 0.99x | 2.30x | 1.12x | 7.60x | 6.84x | 8.76x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
